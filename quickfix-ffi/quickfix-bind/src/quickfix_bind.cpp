#include "quickfix_bind.h"

#include <exception>

#include <quickfix/Application.h>
#include <quickfix/DataDictionary.h>
#include <quickfix/Dictionary.h>
#include <quickfix/FileStore.h>
#include <quickfix/Group.h>
#include <quickfix/Log.h>
#include <quickfix/Message.h>
#include <quickfix/NullStore.h>
#include <quickfix/Session.h>
#include <quickfix/SessionID.h>
#include <quickfix/SessionSettings.h>
#include <quickfix/SocketAcceptor.h>
#include <quickfix/SocketInitiator.h>
#include <quickfix/ThreadedSocketAcceptor.h>
#include <quickfix/ThreadedSocketInitiator.h>

#ifdef HAVE_SSL
#include <quickfix/SSLSocketAcceptor.h>
#include <quickfix/SSLSocketInitiator.h>
#include <quickfix/ThreadedSSLSocketAcceptor.h>
#include <quickfix/ThreadedSSLSocketInitiator.h>
#else

#define _IMPL_CLASS_STUB(_CLS_NAME_, _CLS_IMPLEMENT_)                                                                  \
  class _CLS_NAME_ : public _CLS_IMPLEMENT_ {                                                                          \
  public:                                                                                                              \
    _CLS_NAME_(Application &application, MessageStoreFactory &storeFactory, const SessionSettings &settings,           \
               LogFactory &logFactory)                                                                                 \
        : _CLS_IMPLEMENT_(application, storeFactory, settings, logFactory) {                                           \
      throw new std::logic_error(#_CLS_NAME_ " should not be instantiated.");                                          \
    }                                                                                                                  \
  }

class StubAcceptor : public FIX::Acceptor {
public:
  StubAcceptor(FIX::Application &application, FIX::MessageStoreFactory &storeFactory,
               const FIX::SessionSettings &settings, FIX::LogFactory &logFactory)
      : FIX::Acceptor(application, storeFactory, settings, logFactory) {}
  virtual ~StubAcceptor() {}

private:
  void onStart() override {}
  bool onPoll() override { return false; }
  void onStop() override {}
};

class StubInitiator : public FIX::Initiator {
public:
  StubInitiator(FIX::Application &application, FIX::MessageStoreFactory &storeFactory,
                const FIX::SessionSettings &settings, FIX::LogFactory &logFactory)
      : FIX::Initiator(application, storeFactory, settings, logFactory) {}
  virtual ~StubInitiator() {}

private:
  void onStart() override {}
  bool onPoll() override { return false; }
  void onStop() override {}
  void doConnect(const FIX::SessionID &session, const FIX::Dictionary &dict) override {}
};

namespace FIX {
_IMPL_CLASS_STUB(SSLSocketAcceptor, StubAcceptor);
_IMPL_CLASS_STUB(SSLSocketInitiator, StubInitiator);
_IMPL_CLASS_STUB(ThreadedSSLSocketAcceptor, StubAcceptor);
_IMPL_CLASS_STUB(ThreadedSSLSocketInitiator, StubInitiator);
} // namespace FIX
#endif

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
  } catch (FIX::DataDictionaryNotFound & ex) {                                                                         \
    Fix_setLastError(ex, ERROR_DATA_DICTIONARY_NOT_FOUND);                                                             \
    return (_VAL_);                                                                                                    \
  } catch (FIX::FieldNotFound & ex) {                                                                                  \
    Fix_setLastError(ex, ERROR_FIELD_NOT_FOUND);                                                                       \
    return (_VAL_);                                                                                                    \
  } catch (FIX::FieldConvertError & ex) {                                                                              \
    Fix_setLastError(ex, ERROR_FIELD_CONVERT_ERROR);                                                                   \
    return (_VAL_);                                                                                                    \
  } catch (FIX::MessageParseError & ex) {                                                                              \
    Fix_setLastError(ex, ERROR_MESSAGE_PARSE_ERROR);                                                                   \
    return (_VAL_);                                                                                                    \
  } catch (FIX::InvalidMessage & ex) {                                                                                 \
    Fix_setLastError(ex, ERROR_INVALID_MESSAGE);                                                                       \
    return (_VAL_);                                                                                                    \
  } catch (FIX::ConfigError & ex) {                                                                                    \
    Fix_setLastError(ex, ERROR_CONFIG_ERROR);                                                                          \
    return (_VAL_);                                                                                                    \
  } catch (FIX::RuntimeError & ex) {                                                                                   \
    Fix_setLastError(ex, ERROR_RUNTIME_ERROR);                                                                         \
    return (_VAL_);                                                                                                    \
  } catch (FIX::InvalidTagNumber & ex) {                                                                               \
    Fix_setLastError(ex, ERROR_INVALID_TAG_NUMBER);                                                                    \
    return (_VAL_);                                                                                                    \
  } catch (FIX::RequiredTagMissing & ex) {                                                                             \
    Fix_setLastError(ex, ERROR_REQUIRED_TAG_MISSING);                                                                  \
    return (_VAL_);                                                                                                    \
  } catch (FIX::TagNotDefinedForMessage & ex) {                                                                        \
    Fix_setLastError(ex, ERROR_TAG_NOT_DEFINED_FOR_MESSAGE);                                                           \
    return (_VAL_);                                                                                                    \
  } catch (FIX::NoTagValue & ex) {                                                                                     \
    Fix_setLastError(ex, ERROR_NO_TAG_VALUE);                                                                          \
    return (_VAL_);                                                                                                    \
  } catch (FIX::IncorrectTagValue & ex) {                                                                              \
    Fix_setLastError(ex, ERROR_INCORRECT_TAG_VALUE);                                                                   \
    return (_VAL_);                                                                                                    \
  } catch (FIX::IncorrectDataFormat & ex) {                                                                            \
    Fix_setLastError(ex, ERROR_INCORRECT_DATA_FORMAT);                                                                 \
    return (_VAL_);                                                                                                    \
  } catch (FIX::IncorrectMessageStructure & ex) {                                                                      \
    Fix_setLastError(ex, ERROR_INCORRECT_MESSAGE_STRUCTURE);                                                           \
    return (_VAL_);                                                                                                    \
  } catch (FIX::DuplicateFieldNumber & ex) {                                                                           \
    Fix_setLastError(ex, ERROR_DUPLICATE_FIELD_NUMBER);                                                                \
    return (_VAL_);                                                                                                    \
  } catch (FIX::InvalidMessageType & ex) {                                                                             \
    Fix_setLastError(ex, ERROR_INVALID_MESSAGE_TYPE);                                                                  \
    return (_VAL_);                                                                                                    \
  } catch (FIX::UnsupportedMessageType & ex) {                                                                         \
    Fix_setLastError(ex, ERROR_UNSUPPORTED_MESSAGE_TYPE);                                                              \
    return (_VAL_);                                                                                                    \
  } catch (FIX::UnsupportedVersion & ex) {                                                                             \
    Fix_setLastError(ex, ERROR_UNSUPPORTED_VERSION);                                                                   \
    return (_VAL_);                                                                                                    \
  } catch (FIX::TagOutOfOrder & ex) {                                                                                  \
    Fix_setLastError(ex, ERROR_TAG_OUT_OF_ORDER);                                                                      \
    return (_VAL_);                                                                                                    \
  } catch (FIX::RepeatedTag & ex) {                                                                                    \
    Fix_setLastError(ex, ERROR_REPEATED_TAG);                                                                          \
    return (_VAL_);                                                                                                    \
  } catch (FIX::RepeatingGroupCountMismatch & ex) {                                                                    \
    Fix_setLastError(ex, ERROR_REPEATING_GROUP_COUNT_MISMATCH);                                                        \
    return (_VAL_);                                                                                                    \
  } catch (FIX::DoNotSend & ex) {                                                                                      \
    Fix_setLastError(ex, ERROR_DO_NOT_SEND);                                                                           \
    return (_VAL_);                                                                                                    \
  } catch (FIX::RejectLogon & ex) {                                                                                    \
    Fix_setLastError(ex, ERROR_REJECT_LOGON);                                                                          \
    return (_VAL_);                                                                                                    \
  } catch (FIX::SessionNotFound & ex) {                                                                                \
    Fix_setLastError(ex, ERROR_SESSION_NOT_FOUND);                                                                     \
    return (_VAL_);                                                                                                    \
  } catch (FIX::IOException & ex) {                                                                                    \
    Fix_setLastError(ex, ERROR_IO_EXCEPTION);                                                                          \
    return (_VAL_);                                                                                                    \
  } catch (FIX::SocketSendFailed & ex) {                                                                               \
    Fix_setLastError(ex, ERROR_SOCKET_SEND_FAILED);                                                                    \
    return (_VAL_);                                                                                                    \
  } catch (FIX::SocketRecvFailed & ex) {                                                                               \
    Fix_setLastError(ex, ERROR_SOCKET_RECV_FAILED);                                                                    \
    return (_VAL_);                                                                                                    \
  } catch (FIX::SocketCloseFailed & ex) {                                                                              \
    Fix_setLastError(ex, ERROR_SOCKET_CLOSE_FAILED);                                                                   \
    return (_VAL_);                                                                                                    \
  } catch (FIX::SocketException & ex) {                                                                                \
    Fix_setLastError(ex, ERROR_SOCKET_EXCEPTION);                                                                      \
    return (_VAL_);                                                                                                    \
  } catch (std::exception & e) {                                                                                       \
    Fix_setLastError(e, ERRNO_EXCEPTION);                                                                              \
    return (_VAL_);                                                                                                    \
  }

#define CATCH_OR_RETURN_NULL(_XXX_) CATCH_OR_RETURN(NULL, _XXX_)

#define CATCH_OR_RETURN_ERRNO(_XXX_) CATCH_OR_RETURN(ERRNO_EXCEPTION, _XXX_)

extern "C" {
namespace FIX {

static thread_local char *lastError = nullptr;
static thread_local int8_t lastErrorCode = 0;

static void Fix_setLastError(std::exception &ex, int8_t code) {
  // Release previously set error if any
  Fix_clearLastErrorMessage();

  // Update last error code
  lastErrorCode = code;

  // Get error message and copy it to thread local storage.
  std::string msg = ex.what();
  size_t bufferLen = msg.size() + 1;

  lastError = new char[bufferLen];
  memset(lastError, 0, bufferLen);
  strncpy(lastError, msg.c_str(), msg.size());
}

const char *Fix_getLastErrorMessage() { return lastError; }

int8_t Fix_getLastErrorCode() { return lastErrorCode; }

void Fix_clearLastErrorMessage() {
  if (lastError) {
    delete[] lastError;
    lastError = nullptr;
  }
}

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
    int8_t result = callbacks->toApp(data, &msg, &session);

    if (result == CALLBACK_RESULT_DO_NOT_SEND)
      throw DoNotSend();
  }

  void fromAdmin(const Message &msg, const SessionID &session)
      EXCEPT(FieldNotFound, IncorrectDataFormat, IncorrectTagValue, RejectLogon) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->fromAdmin);
    int8_t result = callbacks->fromAdmin(data, &msg, &session);

    switch (result) {
    case CALLBACK_RESULT_FIELD_NOT_FOUND:
      throw FieldNotFound();
    case CALLBACK_RESULT_INCORRECT_DATA_FORMAT:
      throw IncorrectDataFormat();
    case CALLBACK_RESULT_INCORRECT_TAG_VALUE:
      throw IncorrectTagValue();
    case CALLBACK_RESULT_REJECT_LOGON:
      throw RejectLogon();
    }
  }

  void fromApp(const Message &msg, const SessionID &session)
      EXCEPT(FieldNotFound, IncorrectDataFormat, IncorrectTagValue, UnsupportedMessageType) override {
    RETURN_IF_NULL(callbacks);
    RETURN_IF_NULL(callbacks->fromApp);
    int8_t result = callbacks->fromApp(data, &msg, &session);

    switch (result) {
    case CALLBACK_RESULT_FIELD_NOT_FOUND:
      throw FieldNotFound();
    case CALLBACK_RESULT_INCORRECT_DATA_FORMAT:
      throw IncorrectDataFormat();
    case CALLBACK_RESULT_INCORRECT_TAG_VALUE:
      throw IncorrectTagValue();
    case CALLBACK_RESULT_REJECT_LOGON:
      throw RejectLogon();
    }
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

int8_t FixDictionary_readString(const Dictionary *obj, const char *key, char *buffer, uint64_t buffer_len) {
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
  CATCH_OR_RETURN_ERRNO({ return obj->has(key); });
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

FixMessageStoreFactory_t *FixNullMessageStoreFactory_new() {
  CATCH_OR_RETURN_NULL({ return new NullStoreFactory(); });
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

Acceptor *FixAcceptor_new(Application *application, MessageStoreFactory *storeFactory, const SessionSettings *settings,
                          LogFactory *logFactory, int8_t isMultiThreaded, int8_t isSslEnabled) {
  RETURN_VAL_IF_NULL(application, NULL);
  RETURN_VAL_IF_NULL(storeFactory, NULL);
  RETURN_VAL_IF_NULL(logFactory, NULL);
  RETURN_VAL_IF_NULL(settings, NULL);

  CATCH_OR_RETURN_NULL({
    if (isMultiThreaded && isSslEnabled) {
      return new ThreadedSSLSocketAcceptor(*application, *storeFactory, *settings, *logFactory);
    } else if (isMultiThreaded && !isSslEnabled) {
      return new ThreadedSocketAcceptor(*application, *storeFactory, *settings, *logFactory);
    } else if (!isMultiThreaded && isSslEnabled) {
      return new SSLSocketAcceptor(*application, *storeFactory, *settings, *logFactory);
    } else {
      return new SocketAcceptor(*application, *storeFactory, *settings, *logFactory);
    }
  });
}

int8_t FixAcceptor_start(Acceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->start();
    return 0;
  });
}

int8_t FixAcceptor_block(Acceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->block();
    return 0;
  });
}

int8_t FixAcceptor_poll(Acceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return obj->poll(); });
}

int8_t FixAcceptor_stop(Acceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->stop();
    return 0;
  });
}

int8_t FixAcceptor_isLoggedOn(const Acceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return obj->isLoggedOn(); });
}

int8_t FixAcceptor_isStopped(const Acceptor *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return obj->isStopped(); });
}

FixSession_t *FixAcceptor_getSession(const FixAcceptor_t *obj, const FixSessionID_t *sessionId) {
  RETURN_VAL_IF_NULL(obj, NULL);
  RETURN_VAL_IF_NULL(sessionId, NULL);
  CATCH_OR_RETURN_NULL({ return obj->getSession(*sessionId); });
}

void FixAcceptor_delete(const Acceptor *obj) {
  RETURN_IF_NULL(obj);
  delete obj;
}

Initiator *FixInitiator_new(Application *application, MessageStoreFactory *storeFactory,
                            const SessionSettings *settings, LogFactory *logFactory, int8_t isMultiThreaded,
                            int8_t isSslEnabled) {
  RETURN_VAL_IF_NULL(application, NULL);
  RETURN_VAL_IF_NULL(storeFactory, NULL);
  RETURN_VAL_IF_NULL(logFactory, NULL);
  RETURN_VAL_IF_NULL(settings, NULL);

  CATCH_OR_RETURN_NULL({
    if (isMultiThreaded && isSslEnabled) {
      return new ThreadedSSLSocketInitiator(*application, *storeFactory, *settings, *logFactory);
    } else if (isMultiThreaded && !isSslEnabled) {
      return new ThreadedSocketInitiator(*application, *storeFactory, *settings, *logFactory);
    } else if (!isMultiThreaded && isSslEnabled) {
      return new SSLSocketInitiator(*application, *storeFactory, *settings, *logFactory);
    } else {
      return new SocketInitiator(*application, *storeFactory, *settings, *logFactory);
    }
  });
}

int8_t FixInitiator_start(Initiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->start();
    return 0;
  });
}

int8_t FixInitiator_block(Initiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->block();
    return 0;
  });
}

int8_t FixInitiator_poll(Initiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return obj->poll(); });
}

int8_t FixInitiator_stop(Initiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    obj->stop();
    return 0;
  });
}

int8_t FixInitiator_isLoggedOn(const Initiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return obj->isLoggedOn(); });
}

int8_t FixInitiator_isStopped(const Initiator *obj) {
  RETURN_VAL_IF_NULL(obj, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return obj->isStopped(); });
}

FixSession_t *FixInitiator_getSession(const FixInitiator_t *obj, const FixSessionID_t *sessionId) {
  RETURN_VAL_IF_NULL(obj, NULL);
  RETURN_VAL_IF_NULL(sessionId, NULL);
  CATCH_OR_RETURN_NULL({ return obj->getSession(*sessionId); });
}

void FixInitiator_delete(const Initiator *obj) {
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
  CATCH_OR_RETURN_NULL({ return session->getBeginString().getString().c_str(); })
}

const char *FixSessionID_getSenderCompID(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  CATCH_OR_RETURN_NULL({ return session->getSenderCompID().getString().c_str(); })
}

const char *FixSessionID_getTargetCompID(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  CATCH_OR_RETURN_NULL({ return session->getTargetCompID().getString().c_str(); })
}

const char *FixSessionID_getSessionQualifier(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, NULL);
  CATCH_OR_RETURN_NULL({ return session->getSessionQualifier().c_str(); })
}

int8_t FixSessionID_isFIXT(const SessionID *session) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return session->isFIXT(); });
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

Message *FixMessage_copy(const Message *src) {
  RETURN_VAL_IF_NULL(src, NULL);
  CATCH_OR_RETURN_NULL({ return new Message(*src); });
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

int8_t FixMessage_readString(const FixMessage_t *obj, char *buffer, uint64_t buffer_len) {
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

Header *FixHeader_copy(const Header *src) {
  RETURN_VAL_IF_NULL(src, NULL);
  CATCH_OR_RETURN_NULL({ return new Header(*src); });
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

Trailer *FixTrailer_copy(const Trailer *src) {
  RETURN_VAL_IF_NULL(src, NULL);
  CATCH_OR_RETURN_NULL({ return new Trailer(*src); });
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

Group *FixGroup_copy(const Group *src) {
  RETURN_VAL_IF_NULL(src, NULL);
  CATCH_OR_RETURN_NULL({ return new Group(*src); });
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

FixSession_t *FixSession_lookup(const FixSessionID_t *session_id) {
  RETURN_VAL_IF_NULL(session_id, NULL);
  CATCH_OR_RETURN_NULL({ return Session::lookupSession(*session_id); })
}

int8_t FixSession_logout(FixSession_t *session) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    session->logout();
    return 0;
  });
}

int8_t FixSession_isLoggedOn(FixSession_t *session) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return session->isLoggedOn(); });
}

int8_t FixSession_send(FixSession_t *session, FixMessage_t *msg) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({ return session->send(*msg); });
}

int8_t FixSession_reset(FixSession_t *session) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    session->reset();
    return 0;
  });
}

int8_t FixSession_logon(FixSession_t *session) {
  RETURN_VAL_IF_NULL(session, ERRNO_INVAL);
  CATCH_OR_RETURN_ERRNO({
    session->logon();
    return 0;
  });
}

} // namespace FIX
} // extern C
