#include "quickfix_bind.h"

#include <exception>

#include <quickfix/Application.h>
#include <quickfix/DataDictionary.h>
#include <quickfix/Dictionary.h>
#include <quickfix/FileStore.h>
#include <quickfix/Group.h>
#include <quickfix/Log.h>
#include <quickfix/Message.h>
#include <quickfix/Session.h>
#include <quickfix/SessionID.h>
#include <quickfix/SessionSettings.h>
#include <quickfix/SocketAcceptor.h>
#include <quickfix/SocketInitiator.h>

#ifdef HAVE_MYSQL
#include <quickfix/MySQLStore.h>
#endif // HAVE_MYSQL

#ifdef HAVE_POSTGRESQL
#include <quickfix/PostgreSQLStore.h>
#endif // HAVE_POSTGRESQL

#define RETURN_IF_NULL(_OBJ_)                                                                                          \
  if ((_OBJ_) == nullptr)                                                                                              \
    return;

#define RETURN_VAL_IF_NULL(_OBJ_, _VAL_)                                                                               \
  if ((_OBJ_) == nullptr)                                                                                              \
    return (_VAL_);

#define CATCH_OR_RETURN(_VAL_, _XXX_)                                                                                  \
  try {                                                                                                                \
    _XXX_                                                                                                              \
  } catch (std::exception & e) {                                                                                       \
    if (PRINT_QUICKFIX_EX_STDOUT) {                                                                                    \
      std::cout << "[ERROR: QUICKFIX] " << e.what() << std::endl;                                                      \
    }                                                                                                                  \
    return (_VAL_);                                                                                                    \
  }

#define CATCH_OR_RETURN_NULL(_XXX_) CATCH_OR_RETURN(NULL, _XXX_)

#define CATCH_OR_RETURN_ERRNO(_XXX_) CATCH_OR_RETURN(ERRNO_EXCEPTION, _XXX_)

#define RETURN_CXX_TO_C_STR(_CALL_) CATCH_OR_RETURN_NULL({ return (_CALL_).c_str(); })

#define RETURN_CXX_BOOL_CALL(_CALL_) CATCH_OR_RETURN_ERRNO({ return _CALL_ ? 1 : 0; })

namespace FIX {

class ApplicationBind : public Application {
private:
  const ApplicationCallbacks *callbacks;
  const void *data;

public:
  ApplicationBind(const void *data, const ApplicationCallbacks *callbacks) : callbacks(callbacks), data(data) {}

  ApplicationBind(const ApplicationBind &) = delete;
  ApplicationBind &operator=(const ApplicationBind &) = delete;

  virtual ~ApplicationBind() {}

  void onCreate(const SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onCreate);
    callbacks->onCreate(data, &session);
  }

  void onLogon(const SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onLogon);
    callbacks->onLogon(data, &session);
  }

  void onLogout(const SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onLogout);
    callbacks->onLogout(data, &session);
  }

  void toAdmin(Message &msg, const SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->toAdmin);
    callbacks->toAdmin(data, &msg, &session);
  }

  void toApp(Message &msg, const SessionID &session) EXCEPT(DoNotSend) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->toApp);
    callbacks->toApp(data, &msg, &session);
  }

  void fromAdmin(const Message &msg, const SessionID &session)
      EXCEPT(FieldNotFound, IncorrectDataFormat, IncorrectTagValue, RejectLogon) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->fromAdmin);
    callbacks->fromAdmin(data, &msg, &session);
  }

  void fromApp(const Message &msg, const SessionID &session)
      EXCEPT(FieldNotFound, IncorrectDataFormat, IncorrectTagValue, UnsupportedMessageType) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->fromApp);
    callbacks->fromApp(data, &msg, &session);
  }
};

class ExternalLog : public Log {
private:
  const void *data;
  const SessionID *sessionId;
  const LogCallbacks *callbacks;

public:
  ExternalLog(const void *data, const SessionID *sessionId, const LogCallbacks *callbacks)
      : data(data), sessionId(sessionId), callbacks(callbacks) {}

  ExternalLog(const ExternalLog &) = delete;
  ExternalLog &operator=(const ExternalLog &) = delete;

  virtual ~ExternalLog() {
    if (sessionId) {
      delete sessionId;
    }
  }

  void clear() override {}
  void backup() override {}

  void onIncoming(const std::string &msg) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onIncoming);
    callbacks->onIncoming(data, sessionId, msg.c_str());
  }

  void onOutgoing(const std::string &msg) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onOutgoing);
    callbacks->onOutgoing(data, sessionId, msg.c_str());
  }

  void onEvent(const std::string &msg) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onEvent);
    callbacks->onEvent(data, sessionId, msg.c_str());
  }
};

class ExternalLogFactory : public LogFactory {
private:
  const void *data;
  const LogCallbacks *callbacks;

public:
  ExternalLogFactory(const void *data, const LogCallbacks *callbacks) : data(data), callbacks(callbacks) {}

  ExternalLogFactory(const ExternalLogFactory &) = delete;
  ExternalLogFactory &operator=(const ExternalLogFactory &) = delete;

  virtual ~ExternalLogFactory() {}

  Log *create() override { return new ExternalLog(data, NULL, callbacks); }

  Log *create(const SessionID &sessionId) override {
    auto sessionIdCopy = new SessionID(sessionId);
    return new ExternalLog(data, sessionIdCopy, callbacks);
  }

  void destroy(Log *log) override { delete log; }
};

#ifdef __cplusplus
extern "C" {
#endif

SessionSettings *FixSessionSettings_new() {
  CATCH_OR_RETURN_NULL({ return new SessionSettings(); });
}

SessionSettings *FixSessionSettings_fromPath(const char *configPath) {
  CATCH_OR_RETURN_NULL({ return new SessionSettings(configPath); });
}

const Dictionary *FixSessionSettings_getGlobalRef(const SessionSettings *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return &obj->get(); });
}

const Dictionary *FixSessionSettings_getSessionRef(const SessionSettings *obj, const SessionID *id) {
  RETURN_VAL_IF_NULL(obj, NULL);
  RETURN_VAL_IF_NULL(id, NULL);
  CATCH_OR_RETURN_NULL({ return &obj->get(*id); });
}

int8_t FixSessionSettings_setGlobal(SessionSettings *obj, const Dictionary *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    obj->set(*value);
    return 0;
  })
}

int8_t FixSessionSettings_setSession(SessionSettings *obj, const SessionID *id, const Dictionary *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(id, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    obj->set(*id, *value);
    return 0;
  })
}

void FixSessionSettings_delete(const SessionSettings *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

Dictionary *FixDictionary_new(const char *name) {
  CATCH_OR_RETURN_NULL({ return new Dictionary(name); });
}

int8_t FixDictionary_setString(Dictionary *obj, const char *key, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    obj->setString(key, value);
    return 0;
  })
}

int8_t FixDictionary_setInt(Dictionary *obj, const char *key, int32_t value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    obj->setInt(key, value);
    return 0;
  })
}

int8_t FixDictionary_setDouble(Dictionary *obj, const char *key, double value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    obj->setDouble(key, value);
    return 0;
  })
}

int8_t FixDictionary_setBool(Dictionary *obj, const char *key, int8_t value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    obj->setBool(key, value);
    return 0;
  })
}

int8_t FixDictionary_setDay(Dictionary *obj, const char *key, int32_t value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    obj->setDay(key, value);
    return 0;
  })
}

int64_t FixDictionary_getStringLen(const Dictionary *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({ return obj->getString(key).size() + 1; })
}

int8_t FixDictionary_readString(const Dictionary *obj, const char *key, char *buffer, int64_t buffer_len) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(buffer, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    auto value = obj->getString(key);
    if (buffer_len <= value.size()) {
      return ERRNO_BUFFER_TO_SMALL;
    }

    strncpy(buffer, value.c_str(), buffer_len);
    buffer[value.size()] = '\0';

    return 0;
  })
}

int32_t FixDictionary_getInt(const Dictionary *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, 0);
  RETURN_VAL_IF_NULL(key, 0);

  CATCH_OR_RETURN(0, { return obj->getInt(key); })
}

double FixDictionary_getDouble(const Dictionary *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, 0.0);
  RETURN_VAL_IF_NULL(key, 0.0);

  CATCH_OR_RETURN(0.0, { return obj->getDouble(key); })
}

int8_t FixDictionary_getBool(const Dictionary *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({ return obj->getBool(key); })
}

int32_t FixDictionary_getDay(const Dictionary *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({ return obj->getDay(key); })
}

int8_t FixDictionary_hasKey(const Dictionary *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(obj->has(key));
}

void FixDictionary_delete(const Dictionary *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

DataDictionary *FixDataDictionary_new() {
  CATCH_OR_RETURN_NULL({ return new DataDictionary(); });
}

DataDictionary *FixDataDictionary_fromPath(const char *configPath) {
  CATCH_OR_RETURN_NULL({ return new DataDictionary(configPath); });
}

void FixDataDictionary_delete(const DataDictionary *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

MessageStoreFactory *FixFileMessageStoreFactory_new(const SessionSettings *settings) {
  RETURN_VAL_IF_NULL(settings, NULL);
  CATCH_OR_RETURN_NULL({ return new FileStoreFactory(*settings); });
}

MessageStoreFactory *FixMemoryMessageStoreFactory_new() {
  CATCH_OR_RETURN_NULL({ return new MemoryStoreFactory(); });
}

#ifdef HAVE_MYSQL
MessageStoreFactory *FixMysqlMessageStoreFactory_new(const SessionSettings *settings) {
  RETURN_VAL_IF_NULL(settings, NULL);
  CATCH_OR_RETURN_NULL({ return new MySQLStoreFactory(*settings); });
}
#endif // HAVE_MYSQL

#ifdef HAVE_POSTGRESQL
MessageStoreFactory *FixPostgresMessageStoreFactory_new(const SessionSettings *settings) {
  RETURN_VAL_IF_NULL(settings, NULL);
  CATCH_OR_RETURN_NULL({ return new PostgreSQLStoreFactory(*settings); });
}
#endif // HAVE_POSTGRESQL

void FixMessageStoreFactory_delete(const MessageStoreFactory *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

LogFactory *FixLogFactory_new(const void *data, const LogCallbacks *callbacks) {
  CATCH_OR_RETURN_NULL({ return new ExternalLogFactory(data, callbacks); });
}

void FixLogFactory_delete(const LogFactory *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

Application *FixApplication_new(const void *data, const ApplicationCallbacks *callbacks) {
  RETURN_VAL_IF_NULL(callbacks, NULL);
  CATCH_OR_RETURN_NULL({ return new ApplicationBind(data, callbacks); });
}

void FixApplication_delete(const Application *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

SocketAcceptor *FixSocketAcceptor_new(Application *application, MessageStoreFactory *storeFactory,
                                      const SessionSettings *settings, LogFactory *logFactory) {
  RETURN_VAL_IF_NULL(application, NULL);
  RETURN_VAL_IF_NULL(storeFactory, NULL);
  RETURN_VAL_IF_NULL(logFactory, NULL);
  RETURN_VAL_IF_NULL(settings, NULL);

  CATCH_OR_RETURN_NULL({ return new SocketAcceptor(*application, *storeFactory, *settings, *logFactory); });
}

int8_t FixSocketAcceptor_start(SocketAcceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->start();
    return 0;
  });
}

int8_t FixSocketAcceptor_block(SocketAcceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->block();
    return 0;
  });
}

int8_t FixSocketAcceptor_poll(SocketAcceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(obj->poll());
}

int8_t FixSocketAcceptor_stop(SocketAcceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->stop();
    return 0;
  });
}

int8_t FixSocketAcceptor_isLoggedOn(const SocketAcceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(obj->isLoggedOn());
}

int8_t FixSocketAcceptor_isStopped(const SocketAcceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(obj->isStopped());
}

void FixSocketAcceptor_delete(const SocketAcceptor *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

SocketInitiator *FixSocketInitiator_new(Application *application, MessageStoreFactory *storeFactory,
                                        const SessionSettings *settings, LogFactory *logFactory) {
  RETURN_VAL_IF_NULL(application, NULL);
  RETURN_VAL_IF_NULL(storeFactory, NULL);
  RETURN_VAL_IF_NULL(logFactory, NULL);
  RETURN_VAL_IF_NULL(settings, NULL);

  CATCH_OR_RETURN_NULL({ return new SocketInitiator(*application, *storeFactory, *settings, *logFactory); });
}

int8_t FixSocketInitiator_start(SocketInitiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->start();
    return 0;
  });
}

int8_t FixSocketInitiator_block(SocketInitiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->block();
    return 0;
  });
}

int8_t FixSocketInitiator_poll(SocketInitiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(obj->poll());
}

int8_t FixSocketInitiator_stop(SocketInitiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->stop();
    return 0;
  });
}

int8_t FixSocketInitiator_isLoggedOn(const SocketInitiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(obj->isLoggedOn());
}

int8_t FixSocketInitiator_isStopped(const SocketInitiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(obj->isStopped());
}

void FixSocketInitiator_delete(const SocketInitiator *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

SessionID *FixSessionID_new(const char *beginString, const char *senderCompID, const char *targetCompID,
                            const char *sessionQualifier) {
  RETURN_VAL_IF_NULL(beginString, NULL);
  RETURN_VAL_IF_NULL(senderCompID, NULL);
  RETURN_VAL_IF_NULL(targetCompID, NULL);
  RETURN_VAL_IF_NULL(sessionQualifier, NULL);
  CATCH_OR_RETURN_NULL({ return new SessionID(beginString, senderCompID, targetCompID, sessionQualifier); });
}

SessionID *FixSessionID_copy(const SessionID *src) {
  RETURN_VAL_IF_NULL(src, NULL);
  CATCH_OR_RETURN_NULL({ return new SessionID(*src); });
}

const char *FixSessionID_getBeginString(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(session->getBeginString().getString())
}

const char *FixSessionID_getSenderCompID(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(session->getSenderCompID().getString())
}

const char *FixSessionID_getTargetCompID(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(session->getTargetCompID().getString())
}

const char *FixSessionID_getSessionQualifier(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(session->getSessionQualifier())
}

int8_t FixSessionID_isFIXT(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(session->isFIXT());
}

const char *FixSessionID_toString(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  CATCH_OR_RETURN_NULL({ return session->toStringFrozen().c_str(); });
}

void FixSessionID_delete(const SessionID *session) {
  RETURN_IF_NULL(session);
  delete session;
}

Message *FixMessage_new() {
  CATCH_OR_RETURN_NULL({ return new Message(); });
}

Message *FixMessage_fromString(const char *text) {
  RETURN_VAL_IF_NULL(text, NULL);
  CATCH_OR_RETURN_NULL({ return new Message(text, /* validate = */ false); });
}

Message *FixMessage_fromStringAndDictionary(const char *text, const DataDictionary *dictionary) {
  RETURN_VAL_IF_NULL(text, NULL);
  RETURN_VAL_IF_NULL(dictionary, NULL);

  CATCH_OR_RETURN_NULL({ return new Message(text, *dictionary, /* validate = */ true); });
}

const char *FixMessage_getField(const Message *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return obj->getField(tag).c_str(); });
}

int8_t FixMessage_setField(Message *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->setField(tag, value);
    return 0;
  });
}

int8_t FixMessage_removeField(Message *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->removeField(tag);
    return 0;
  });
}

int8_t FixMessage_addGroup(Message *obj, const Group *group) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(group, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->addGroup(*group);
    return 0;
  })
}

int64_t FixMessage_getStringLen(const FixMessage_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({ return obj->toString().size() + 1; });
}

int8_t FixMessage_readString(const FixMessage_t *obj, char *buffer, int64_t buffer_len) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(buffer, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    auto value = obj->toString();
    if (buffer_len <= value.size()) {
      return ERRNO_BUFFER_TO_SMALL;
    }

    strncpy(buffer, value.c_str(), buffer_len);
    buffer[value.size()] = '\0';

    return 0;
  })
}

void FixMessage_delete(const Message *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

Header *FixHeader_new() {
  CATCH_OR_RETURN_NULL({ return new Header(); });
}

Header *FixMessage_copyHeader(const Message *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return new Header(obj->getHeader()); });
}

Header *FixMessage_getHeaderRef(Message *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return &obj->getHeader(); });
}

const char *FixHeader_getField(const Header *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return obj->getField(tag).c_str(); });
}

int8_t FixHeader_setField(Header *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->setField(tag, value);
    return 0;
  });
}

int8_t FixHeader_removeField(Header *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->removeField(tag);
    return 0;
  });
}

int8_t FixHeader_addGroup(Header *obj, const Group *group) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(group, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->addGroup(*group);
    return 0;
  })
}

void FixHeader_delete(const Header *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

Trailer *FixTrailer_new() {
  CATCH_OR_RETURN_NULL({ return new Trailer(); });
}

Trailer *FixMessage_copyTrailer(const Message *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return new Trailer(obj->getTrailer()); });
}

Trailer *FixMessage_getTrailerRef(Message *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return &obj->getTrailer(); });
}

const char *FixTrailer_getField(const Trailer *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return obj->getField(tag).c_str(); });
}

int8_t FixTrailer_setField(Trailer *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->setField(tag, value);
    return 0;
  });
}

int8_t FixTrailer_removeField(Trailer *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->removeField(tag);
    return 0;
  });
}

int8_t FixTrailer_addGroup(Trailer *obj, const Group *group) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(group, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->addGroup(*group);
    return 0;
  })
}

void FixTrailer_delete(const Trailer *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

Group *FixGroup_new(int32_t fieldId, int32_t delim, const int32_t order[]) {
  RETURN_VAL_IF_NULL(order, NULL);
  CATCH_OR_RETURN_NULL({ return new Group(fieldId, delim, order); });
}

Group *FixMessage_copyGroup(const Message *obj, int32_t num, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto src_group = static_cast<Group *>(obj->getGroupPtr(num, tag));
    return new Group(*src_group);
  });
}

Group *FixHeader_copyGroup(const Header *obj, int32_t num, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto src_group = static_cast<Group *>(obj->getGroupPtr(num, tag));
    return new Group(*src_group);
  });
}

Group *FixTrailer_copyGroup(const Trailer *obj, int32_t num, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto src_group = static_cast<Group *>(obj->getGroupPtr(num, tag));
    return new Group(*src_group);
  });
}

Group *FixGroup_copyGroup(const Group *obj, int32_t num, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto src_group = static_cast<Group *>(obj->getGroupPtr(num, tag));
    return new Group(*src_group);
  });
}

Group *FixMessage_getGroupRef(const Message *obj, int32_t num, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return static_cast<Group *>(obj->getGroupPtr(num, tag)); });
}

int32_t FixGroup_getFieldId(const Group *obj) {
  RETURN_VAL_IF_NULL(obj, 0);
  CATCH_OR_RETURN(0, { return obj->field(); });
}

int32_t FixGroup_getDelim(const Group *obj) {
  RETURN_VAL_IF_NULL(obj, 0);
  CATCH_OR_RETURN(0, { return obj->delim(); });
}

const char *FixGroup_getField(const Group *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({ return obj->getField(tag).c_str(); });
}

int8_t FixGroup_setField(Group *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->setField(tag, value);
    return 0;
  });
}

int8_t FixGroup_removeField(Group *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->removeField(tag);
    return 0;
  });
}

int8_t FixGroup_addGroup(Group *obj, const Group *group) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(group, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->addGroup(*group);
    return 0;
  })
}

void FixGroup_delete(const Group *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

int8_t FixSession_sendToTarget(Message *msg, const SessionID *session_id) {
  RETURN_VAL_IF_NULL(msg, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(session_id, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    Session::sendToTarget(*msg, *session_id);
    return 0;
  });
}

} // namespace FIX

#ifdef __cplusplus
}
#endif // __cplusplus
