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

#define DELETE_OBJ(_TYPE_, _OBJ_)                                                                                      \
  {                                                                                                                    \
    auto fix_obj = (_TYPE_ *)(_OBJ_);                                                                                  \
    delete fix_obj;                                                                                                    \
  }

#define RETURN_CXX_TO_C_STR(_TYPE_, _OBJ_, _METHOD_)                                                                   \
  CATCH_OR_RETURN_NULL({                                                                                               \
    auto fix_obj = (_TYPE_ *)((_OBJ_));                                                                                \
    return fix_obj->_METHOD_.c_str();                                                                                  \
  })

#define SAFE_CXX_CALL(_TYPE_, _OBJ_, _METHOD_)                                                                         \
  CATCH_OR_RETURN_ERRNO({                                                                                              \
    auto fix_obj = (_TYPE_ *)(_OBJ_);                                                                                  \
    fix_obj->_METHOD_;                                                                                                 \
    return 0;                                                                                                          \
  })

#define RETURN_CXX_BOOL_CALL(_TYPE_, _OBJ_, _METHOD_)                                                                  \
  CATCH_OR_RETURN_ERRNO({                                                                                              \
    auto fix_obj = (_TYPE_ *)(_OBJ_);                                                                                  \
    return fix_obj->_METHOD_ ? 1 : 0;                                                                                  \
  })

class ApplicationBind : public FIX::Application {
private:
  const FixApplicationCallbacks_t *callbacks;
  const void *data;

public:
  ApplicationBind(const void *data, const FixApplicationCallbacks_t *callbacks) : callbacks(callbacks), data(data) {}

  ApplicationBind(const ApplicationBind &) = delete;
  ApplicationBind &operator=(const ApplicationBind &) = delete;

  virtual ~ApplicationBind() {}

  void onCreate(const FIX::SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onCreate);
    callbacks->onCreate(data, (FixSessionID_t *)(&session));
  }

  void onLogon(const FIX::SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onLogon);
    callbacks->onLogon(data, (FixSessionID_t *)(&session));
  }

  void onLogout(const FIX::SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->onLogout);
    callbacks->onLogout(data, (FixSessionID_t *)(&session));
  }

  void toAdmin(FIX::Message &msg, const FIX::SessionID &session) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->toAdmin);
    callbacks->toAdmin(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
  }

  void toApp(FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::DoNotSend) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->toApp);
    callbacks->toApp(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
  }

  void fromAdmin(const FIX::Message &msg, const FIX::SessionID &session)
      EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue, FIX::RejectLogon) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->fromAdmin);
    callbacks->fromAdmin(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
  }

  void fromApp(const FIX::Message &msg, const FIX::SessionID &session)
      EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue,
             FIX::UnsupportedMessageType) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->fromApp);
    callbacks->fromApp(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
  }
};

class ExternalLog : public FIX::Log {
private:
  const void *data;
  const FixSessionID_t *sessionId;
  const FixLogCallbacks_t *callbacks;

public:
  ExternalLog(const void *data, const FixSessionID_t *sessionId, const FixLogCallbacks_t *callbacks)
      : data(data), sessionId(sessionId), callbacks(callbacks) {}

  ExternalLog(const ExternalLog &) = delete;
  ExternalLog &operator=(const ExternalLog &) = delete;

  virtual ~ExternalLog() {
    auto fixSessionId = (FIX::SessionID *)(sessionId);
    if (fixSessionId) {
      delete fixSessionId;
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

class ExternalLogFactory : public FIX::LogFactory {
private:
  const void *data;
  const FixLogCallbacks_t *callbacks;

public:
  ExternalLogFactory(const void *data, const FixLogCallbacks_t *callbacks) : data(data), callbacks(callbacks) {}

  ExternalLogFactory(const ExternalLogFactory &) = delete;
  ExternalLogFactory &operator=(const ExternalLogFactory &) = delete;

  virtual ~ExternalLogFactory() {}

  FIX::Log *create() override { return new ExternalLog(data, NULL, callbacks); }

  FIX::Log *create(const FIX::SessionID &sessionId) override {
    auto sessionIdCopy = new FIX::SessionID(sessionId);
    return new ExternalLog(data, (FixSessionID_t *)sessionIdCopy, callbacks);
  }

  void destroy(FIX::Log *log) override { delete log; }
};

#ifdef __cplusplus
extern "C" {
#endif

FixSessionSettings_t *FixSessionSettings_new() {
  CATCH_OR_RETURN_NULL({ return (FixSessionSettings_t *)(new FIX::SessionSettings()); });
}

FixSessionSettings_t *FixSessionSettings_fromPath(const char *configPath) {
  CATCH_OR_RETURN_NULL({ return (FixSessionSettings_t *)(new FIX::SessionSettings(configPath)); });
}

void FixSessionSettings_delete(FixSessionSettings_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::SessionSettings, obj);
}

FixDictionary_t *FixDictionary_new(const char *name) {
  CATCH_OR_RETURN_NULL({ return (FixDictionary_t *)(new FIX::Dictionary(name)); });
}

int8_t FixDictionary_setString(const FixDictionary_t *obj, const char *key, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({
    fix_obj->setString(key, value);
    return 0;
  })
}

int8_t FixDictionary_setInt(const FixDictionary_t *obj, const char *key, int32_t value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({
    fix_obj->setInt(key, value);
    return 0;
  })
}

int8_t FixDictionary_setDouble(const FixDictionary_t *obj, const char *key, double value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({
    fix_obj->setDouble(key, value);
    return 0;
  })
}

int8_t FixDictionary_setBool(const FixDictionary_t *obj, const char *key, int8_t value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({
    fix_obj->setBool(key, value);
    return 0;
  })
}

int8_t FixDictionary_setDay(const FixDictionary_t *obj, const char *key, int32_t value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({
    fix_obj->setDay(key, value);
    return 0;
  })
}

int64_t FixDictionary_getStringLen(const FixDictionary_t *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({ return fix_obj->getString(key).size(); })
}

int8_t FixDictionary_readString(const FixDictionary_t *obj, const char *key, char *buffer, int64_t buffer_len) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({
    auto value = fix_obj->getString(key);
    if (buffer_len <= value.length()) {
      return ERRNO_BUFFER_TO_SMALL;
    }

    strncpy(buffer, value.c_str(), buffer_len);
    buffer[value.length()] = '\0';

    return 0;
  })
}

int32_t FixDictionary_getInt(const FixDictionary_t *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, 0);
  RETURN_VAL_IF_NULL(key, 0);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN(0, { return fix_obj->getInt(key); })
}

double FixDictionary_getDouble(const FixDictionary_t *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, 0.0);
  RETURN_VAL_IF_NULL(key, 0.0);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN(0.0, { return fix_obj->getDouble(key); })
}

int8_t FixDictionary_getBool(const FixDictionary_t *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({ return fix_obj->getBool(key); })
}

int32_t FixDictionary_getDay(const FixDictionary_t *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);

  auto fix_obj = (FIX::Dictionary *)(obj);
  CATCH_OR_RETURN_ERRNO({ return fix_obj->getDay(key); })
}

int8_t FixDictionary_hasKey(const FixDictionary_t *obj, const char *key) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(key, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::Dictionary, obj, has(key));
}

void FixDictionary_delete(FixDictionary_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::Dictionary, obj);
}

FixDataDictionary_t *FixDataDictionary_new() {
  CATCH_OR_RETURN_NULL({ return (FixDataDictionary_t *)(new FIX::DataDictionary()); });
}

FixDataDictionary_t *FixDataDictionary_fromPath(const char *configPath) {
  CATCH_OR_RETURN_NULL({ return (FixDataDictionary_t *)(new FIX::DataDictionary(configPath)); });
}

void FixDataDictionary_delete(FixDataDictionary_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::DataDictionary, obj);
}

FixFileStoreFactory_t *FixFileStoreFactory_new(const FixSessionSettings_t *settings) {
  RETURN_VAL_IF_NULL(settings, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_settings = (FIX::SessionSettings *)(settings);
    return (FixFileStoreFactory_t *)(new FIX::FileStoreFactory(*fix_settings));
  });
}

void FixFileStoreFactory_delete(FixFileStoreFactory_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::FileStoreFactory, obj);
}

FixLogFactory_t *FixLogFactory_new(const void *data, const FixLogCallbacks_t *callbacks) {
  CATCH_OR_RETURN_NULL({ return (FixLogFactory_t *)(new ExternalLogFactory(data, callbacks)); });
}

void FixLogFactory_delete(FixLogFactory_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(ExternalLogFactory, obj);
}

FixApplication_t *FixApplication_new(const void *data, const FixApplicationCallbacks_t *callbacks) {
  RETURN_VAL_IF_NULL(callbacks, NULL);
  CATCH_OR_RETURN_NULL({ return (FixApplication_t *)(new ApplicationBind(data, callbacks)); });
}

void FixApplication_delete(FixApplication_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(ApplicationBind, obj);
}

FixSocketAcceptor_t *FixSocketAcceptor_new(const FixApplication_t *application,
                                           const FixFileStoreFactory_t *storeFactory,
                                           const FixSessionSettings_t *settings, const FixLogFactory_t *logFactory) {
  RETURN_VAL_IF_NULL(application, NULL);
  RETURN_VAL_IF_NULL(storeFactory, NULL);
  RETURN_VAL_IF_NULL(logFactory, NULL);
  RETURN_VAL_IF_NULL(settings, NULL);

  auto fix_application = (ApplicationBind *)(application);
  auto fix_store_factory = (FIX::FileStoreFactory *)(storeFactory);
  auto fix_log_factory = (ExternalLogFactory *)(logFactory);
  auto fix_settings = (FIX::SessionSettings *)(settings);

  CATCH_OR_RETURN_NULL({
    return (FixSocketAcceptor_t *)(new FIX::SocketAcceptor(*fix_application, *fix_store_factory, *fix_settings,
                                                           *fix_log_factory));
  });
}

int8_t FixSocketAcceptor_start(const FixSocketAcceptor_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::SocketAcceptor, obj, start());
}

int8_t FixSocketAcceptor_block(const FixSocketAcceptor_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::SocketAcceptor, obj, block());
}

int8_t FixSocketAcceptor_poll(const FixSocketAcceptor_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::SocketAcceptor, obj, poll());
}

int8_t FixSocketAcceptor_stop(const FixSocketAcceptor_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::SocketAcceptor, obj, stop());
}

int8_t FixSocketAcceptor_isLoggedOn(const FixSocketAcceptor_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::SocketAcceptor, obj, isLoggedOn());
}

int8_t FixSocketAcceptor_isStopped(const FixSocketAcceptor_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::SocketAcceptor, obj, isStopped());
}

void FixSocketAcceptor_delete(FixSocketAcceptor_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::SocketAcceptor, obj);
}

FixSocketInitiator_t *FixSocketInitiator_new(const FixApplication_t *application,
                                             const FixFileStoreFactory_t *storeFactory,
                                             const FixSessionSettings_t *settings, const FixLogFactory_t *logFactory) {
  RETURN_VAL_IF_NULL(application, NULL);
  RETURN_VAL_IF_NULL(storeFactory, NULL);
  RETURN_VAL_IF_NULL(logFactory, NULL);
  RETURN_VAL_IF_NULL(settings, NULL);

  auto fix_application = (ApplicationBind *)(application);
  auto fix_store_factory = (FIX::FileStoreFactory *)(storeFactory);
  auto fix_log_factory = (ExternalLogFactory *)(logFactory);
  auto fix_settings = (FIX::SessionSettings *)(settings);

  CATCH_OR_RETURN_NULL({
    return (FixSocketInitiator_t *)(new FIX::SocketInitiator(*fix_application, *fix_store_factory, *fix_settings,
                                                             *fix_log_factory));
  });
}

int8_t FixSocketInitiator_start(const FixSocketInitiator_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::SocketInitiator, obj, start());
}

int8_t FixSocketInitiator_block(const FixSocketInitiator_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::SocketInitiator, obj, block());
}

int8_t FixSocketInitiator_poll(const FixSocketInitiator_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::SocketInitiator, obj, poll());
}

int8_t FixSocketInitiator_stop(const FixSocketInitiator_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::SocketInitiator, obj, stop());
}

int8_t FixSocketInitiator_isLoggedOn(const FixSocketInitiator_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::SocketInitiator, obj, isLoggedOn());
}

int8_t FixSocketInitiator_isStopped(const FixSocketInitiator_t *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::SocketInitiator, obj, isStopped());
}

void FixSocketInitiator_delete(FixSocketInitiator_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::SocketInitiator, obj);
}

FixSessionID_t *FixSessionID_new(const char *beginString, const char *senderCompID, const char *targetCompID,
                                 const char *sessionQualifier) {
  RETURN_VAL_IF_NULL(beginString, NULL);
  RETURN_VAL_IF_NULL(senderCompID, NULL);
  RETURN_VAL_IF_NULL(targetCompID, NULL);
  RETURN_VAL_IF_NULL(sessionQualifier, NULL);
  CATCH_OR_RETURN_NULL(
      { return (FixSessionID_t *)(new FIX::SessionID(beginString, senderCompID, targetCompID, sessionQualifier)); });
}

FixSessionID_t *FixSessionID_copy(const FixSessionID_t *src) {
  RETURN_VAL_IF_NULL(src, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_obj = (FIX::SessionID *)(src);
    return (FixSessionID_t *)(new FIX::SessionID(*fix_obj));
  });
}

const char *FixSessionID_getBeginString(const FixSessionID_t *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(FIX::SessionID, session, getBeginString().getString())
}

const char *FixSessionID_getSenderCompID(const FixSessionID_t *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(FIX::SessionID, session, getSenderCompID().getString())
}

const char *FixSessionID_getTargetCompID(const FixSessionID_t *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(FIX::SessionID, session, getTargetCompID().getString())
}

const char *FixSessionID_getSessionQualifier(const FixSessionID_t *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  RETURN_CXX_TO_C_STR(FIX::SessionID, session, getSessionQualifier())
}

int8_t FixSessionID_isFIXT(const FixSessionID_t *session) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  RETURN_CXX_BOOL_CALL(FIX::SessionID, session, isFIXT());
}

const char *FixSessionID_toString(const FixSessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_obj = (FIX::SessionID *)(session);
    return fix_obj->toStringFrozen().c_str();
  });
}

void FixSessionID_delete(FixSessionID_t *session) {
  RETURN_IF_NULL(session);
  DELETE_OBJ(FIX::SessionID, session);
}

FixMessage_t *FixMessage_new() {
  CATCH_OR_RETURN_NULL({ return (FixMessage_t *)(new FIX::Message()); });
}

FixMessage_t *FixMessage_fromString(const char *text) {
  RETURN_VAL_IF_NULL(text, NULL);
  CATCH_OR_RETURN_NULL({ return (FixMessage_t *)(new FIX::Message(text, /* validate = */ false)); });
}

FixMessage_t *FixMessage_fromStringAndDictionary(const char *text, const FixDataDictionary_t *dictionary) {
  RETURN_VAL_IF_NULL(text, NULL);
  RETURN_VAL_IF_NULL(dictionary, NULL);

  auto fix_dictionary = (const FIX::DataDictionary *)(dictionary);
  CATCH_OR_RETURN_NULL({ return (FixMessage_t *)(new FIX::Message(text, *fix_dictionary, /* validate = */ true)); });
}

const char *FixMessage_getField(const FixMessage_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_obj = (FIX::Message *)(obj);
    return fix_obj->getField(tag).c_str();
  });
}

int8_t FixMessage_setField(const FixMessage_t *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Message, obj, setField(tag, value));
}

int8_t FixMessage_removeField(const FixMessage_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Message, obj, removeField(tag));
}

int8_t FixMessage_toBuffer(const FixMessage_t *obj, char *buffer, size_t length) {
  if (length == 0)
    return 0;

  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(buffer, ERRNO_INVAL);

  CATCH_OR_RETURN_ERRNO({
    auto fix_obj = (FIX::Message *)(obj);

    auto repr = fix_obj->toString();
    if (length <= repr.length()) {
      return ERRNO_BUFFER_TO_SMALL;
    }

    strncpy(buffer, repr.c_str(), length);
    buffer[repr.length()] = '\0';

    return 0;
  });
}

void FixMessage_delete(FixMessage_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::Message, obj);
}

FixHeader_t *FixMessage_copyHeader(const FixMessage_t *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  auto fix_obj = (FIX::Message *)(obj);
  CATCH_OR_RETURN_NULL({ return (FixHeader_t *)(new FIX::Header(fix_obj->getHeader())); });
}

FixHeader_t *FixMessage_getHeaderRef(const FixMessage_t *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  auto fix_obj = (FIX::Message *)(obj);
  CATCH_OR_RETURN_NULL({ return (FixHeader_t *)(&fix_obj->getHeader()); });
}

const char *FixHeader_getField(const FixHeader_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  auto fix_obj = (FIX::Header *)(obj);
  CATCH_OR_RETURN_NULL({ return fix_obj->getField(tag).c_str(); });
}

int8_t FixHeader_setField(const FixHeader_t *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Header, obj, setField(tag, value));
}

int8_t FixHeader_removeField(const FixHeader_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Header, obj, removeField(tag));
}

void FixHeader_delete(FixHeader_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::Header, obj);
}

FixTrailer_t *FixMessage_copyTrailer(const FixMessage_t *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  auto fix_obj = (FIX::Message *)(obj);
  CATCH_OR_RETURN_NULL({ return (FixTrailer_t *)(new FIX::Trailer(fix_obj->getTrailer())); });
}

FixTrailer_t *FixMessage_getTrailerRef(const FixMessage_t *obj) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_obj = (FIX::Message *)(obj);
    return (FixTrailer_t *)(&fix_obj->getTrailer());
  });
}

const char *FixTrailer_getField(const FixTrailer_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_obj = (FIX::Trailer *)(obj);
    return fix_obj->getField(tag).c_str();
  });
}

int8_t FixTrailer_setField(const FixTrailer_t *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Trailer, obj, setField(tag, value));
}

int8_t FixTrailer_removeField(const FixTrailer_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Trailer, obj, removeField(tag));
}

void FixTrailer_delete(FixTrailer_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::Trailer, obj);
}

FixGroup_t *FixMessage_copyGroup(const FixMessage_t *obj, int32_t num, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  auto fix_obj = (FIX::Message *)(obj);
  CATCH_OR_RETURN_NULL({
    auto src_group = (FIX::Group *)(fix_obj->getGroupPtr(num, tag));
    return (FixGroup_t *)(new FIX::Group(*src_group));
  });
}

FixGroup_t *FixMessage_getGroupRef(const FixMessage_t *obj, int32_t num, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_obj = (FIX::Message *)(obj);
    return (FixGroup_t *)(fix_obj->getGroupPtr(num, tag));
  });
}

const char *FixGroup_getField(const FixGroup_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, NULL);
  CATCH_OR_RETURN_NULL({
    auto fix_obj = (FIX::Group *)(obj);
    return fix_obj->getField(tag).c_str();
  });
}

int8_t FixGroup_setField(const FixGroup_t *obj, int32_t tag, const char *value) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(value, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Group, obj, setField(tag, value));
}

int8_t FixGroup_removeField(const FixGroup_t *obj, int32_t tag) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  SAFE_CXX_CALL(FIX::Group, obj, removeField(tag));
}

void FixGroup_delete(FixGroup_t *obj) {
  RETURN_IF_NULL(obj);
  DELETE_OBJ(FIX::Group, obj);
}

int8_t FixSession_sendToTarget(const FixMessage_t *msg, const FixSessionID_t *session_id) {
  RETURN_VAL_IF_NULL(msg, ERRNO_INVAL);
  RETURN_VAL_IF_NULL(session_id, ERRNO_INVAL);

  auto fix_msg = (FIX::Message *)(msg);
  auto fix_session_id = (FIX::SessionID *)(session_id);

  CATCH_OR_RETURN_ERRNO({
    FIX::Session::sendToTarget(*fix_msg, *fix_session_id);
    return 0;
  });
}

#ifdef __cplusplus
}
#endif // __cplusplus
