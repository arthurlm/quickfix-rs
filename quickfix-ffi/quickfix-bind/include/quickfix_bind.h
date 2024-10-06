#ifndef _QUICKFIX_BIND_H_
#define _QUICKFIX_BIND_H_

#include <stddef.h>
#include <stdint.h>

// #define HAVE_MYSQL
// #define HAVE_POSTGRESQL

#define ERRNO_INVAL -1
#define ERRNO_EXCEPTION -2
#define ERRNO_BUFFER_TO_SMALL -3

#define ERROR_DATA_DICTIONARY_NOT_FOUND -10
#define ERROR_FIELD_NOT_FOUND -11
#define ERROR_FIELD_CONVERT_ERROR -12
#define ERROR_MESSAGE_PARSE_ERROR -13
#define ERROR_INVALID_MESSAGE -14
#define ERROR_CONFIG_ERROR -15
#define ERROR_RUNTIME_ERROR -16
#define ERROR_INVALID_TAG_NUMBER -17
#define ERROR_REQUIRED_TAG_MISSING -18
#define ERROR_TAG_NOT_DEFINED_FOR_MESSAGE -19
#define ERROR_NO_TAG_VALUE -20
#define ERROR_INCORRECT_TAG_VALUE -21
#define ERROR_INCORRECT_DATA_FORMAT -22
#define ERROR_INCORRECT_MESSAGE_STRUCTURE -23
#define ERROR_DUPLICATE_FIELD_NUMBER -24
#define ERROR_INVALID_MESSAGE_TYPE -25
#define ERROR_UNSUPPORTED_MESSAGE_TYPE -26
#define ERROR_UNSUPPORTED_VERSION -27
#define ERROR_TAG_OUT_OF_ORDER -28
#define ERROR_REPEATED_TAG -29
#define ERROR_REPEATING_GROUP_COUNT_MISMATCH -30
#define ERROR_DO_NOT_SEND -31
#define ERROR_REJECT_LOGON -32
#define ERROR_SESSION_NOT_FOUND -33
#define ERROR_IO_EXCEPTION -34
#define ERROR_SOCKET_EXCEPTION -35
#define ERROR_SOCKET_SEND_FAILED -36
#define ERROR_SOCKET_RECV_FAILED -37
#define ERROR_SOCKET_CLOSE_FAILED -38

#define CALLBACK_OK 0
#define CALLBACK_RESULT_DO_NOT_SEND -1
#define CALLBACK_RESULT_FIELD_NOT_FOUND -2
#define CALLBACK_RESULT_INCORRECT_DATA_FORMAT -3
#define CALLBACK_RESULT_INCORRECT_TAG_VALUE -4
#define CALLBACK_RESULT_REJECT_LOGON -5
#define CALLBACK_RESULT_UNSUPPORTED_MESSAGE_TYPE -6

#ifdef __cplusplus
extern "C" {
namespace FIX {
#endif

typedef struct SessionSettings FixSessionSettings_t;
typedef struct Dictionary FixDictionary_t;
typedef struct DataDictionary FixDataDictionary_t;
typedef struct MessageStoreFactory FixMessageStoreFactory_t;
typedef struct LogFactory FixLogFactory_t;
typedef struct Application FixApplication_t;
typedef struct SocketAcceptor FixSocketAcceptor_t;
typedef struct SocketInitiator FixSocketInitiator_t;
typedef struct Session FixSession_t;
typedef struct SessionID FixSessionID_t;
typedef struct Message FixMessage_t;
typedef struct Header FixHeader_t;
typedef struct Trailer FixTrailer_t;
typedef struct Group FixGroup_t;

typedef struct ApplicationCallbacks {
  void (*onCreate)(const void *data, const FixSessionID_t *session);
  void (*onLogon)(const void *data, const FixSessionID_t *session);
  void (*onLogout)(const void *data, const FixSessionID_t *session);
  void (*toAdmin)(const void *data, FixMessage_t *msg, const FixSessionID_t *session);
  int8_t (*toApp)(const void *data, FixMessage_t *msg, const FixSessionID_t *session);
  int8_t (*fromAdmin)(const void *data, const FixMessage_t *msg, const FixSessionID_t *session);
  int8_t (*fromApp)(const void *data, const FixMessage_t *msg, const FixSessionID_t *session);
} FixApplicationCallbacks_t;

typedef struct LogCallbacks {
  void (*onIncoming)(const void *data, const FixSessionID_t *sessionId, const char *msg);
  void (*onOutgoing)(const void *data, const FixSessionID_t *sessionId, const char *msg);
  void (*onEvent)(const void *data, const FixSessionID_t *sessionId, const char *msg);
} FixLogCallbacks_t;

const char *Fix_getLastErrorMessage();
int8_t Fix_getLastErrorCode();
void Fix_clearLastErrorMessage();

FixSessionSettings_t *FixSessionSettings_new();
FixSessionSettings_t *FixSessionSettings_fromPath(const char *configPath);
const FixDictionary_t *FixSessionSettings_getGlobalRef(const FixSessionSettings_t *obj);
const FixDictionary_t *FixSessionSettings_getSessionRef(const FixSessionSettings_t *obj, const FixSessionID_t *id);
int8_t FixSessionSettings_setGlobal(FixSessionSettings_t *obj, const FixDictionary_t *value);
int8_t FixSessionSettings_setSession(FixSessionSettings_t *obj, const FixSessionID_t *id, const FixDictionary_t *value);
void FixSessionSettings_delete(const FixSessionSettings_t *obj);

FixDictionary_t *FixDictionary_new(const char *name);
int8_t FixDictionary_setString(FixDictionary_t *obj, const char *key, const char *value);
int8_t FixDictionary_setInt(FixDictionary_t *obj, const char *key, int32_t value);
int8_t FixDictionary_setDouble(FixDictionary_t *obj, const char *key, double value);
int8_t FixDictionary_setBool(FixDictionary_t *obj, const char *key, int8_t value);
int8_t FixDictionary_setDay(FixDictionary_t *obj, const char *key, int32_t value);
int64_t FixDictionary_getStringLen(const FixDictionary_t *obj, const char *key);
int8_t FixDictionary_readString(const FixDictionary_t *obj, const char *key, char *buffer, uint64_t buffer_len);
int32_t FixDictionary_getInt(const FixDictionary_t *obj, const char *key);
double FixDictionary_getDouble(const FixDictionary_t *obj, const char *key);
int8_t FixDictionary_getBool(const FixDictionary_t *obj, const char *key);
int32_t FixDictionary_getDay(const FixDictionary_t *obj, const char *key);
int8_t FixDictionary_hasKey(const FixDictionary_t *obj, const char *key);
void FixDictionary_delete(const FixDictionary_t *obj);

FixDataDictionary_t *FixDataDictionary_new();
FixDataDictionary_t *FixDataDictionary_fromPath(const char *configPath);
void FixDataDictionary_delete(const FixDataDictionary_t *obj);

FixMessageStoreFactory_t *FixFileMessageStoreFactory_new(const FixSessionSettings_t *settings);
FixMessageStoreFactory_t *FixMemoryMessageStoreFactory_new();
FixMessageStoreFactory_t *FixNullMessageStoreFactory_new();

#ifdef HAVE_MYSQL
FixMessageStoreFactory_t *FixMysqlMessageStoreFactory_new(const FixSessionSettings_t *settings);
#endif // HAVE_MYSQL

#ifdef HAVE_POSTGRESQL
FixMessageStoreFactory_t *FixPostgresMessageStoreFactory_new(const FixSessionSettings_t *settings);
#endif // HAVE_POSTGRESQL

void FixMessageStoreFactory_delete(const FixMessageStoreFactory_t *obj);

FixLogFactory_t *FixLogFactory_new(const void *data, const FixLogCallbacks_t *callbacks);
void FixLogFactory_delete(const FixLogFactory_t *obj);

FixApplication_t *FixApplication_new(const void *data, const FixApplicationCallbacks_t *callbacks);
void FixApplication_delete(const FixApplication_t *obj);

FixSocketAcceptor_t *FixSocketAcceptor_new(FixApplication_t *application, FixMessageStoreFactory_t *storeFactory,
                                           const FixSessionSettings_t *settings, FixLogFactory_t *logFactory);
int8_t FixSocketAcceptor_start(FixSocketAcceptor_t *obj);
int8_t FixSocketAcceptor_block(FixSocketAcceptor_t *obj);
int8_t FixSocketAcceptor_poll(FixSocketAcceptor_t *obj);
int8_t FixSocketAcceptor_stop(FixSocketAcceptor_t *obj);
int8_t FixSocketAcceptor_isLoggedOn(const FixSocketAcceptor_t *obj);
int8_t FixSocketAcceptor_isStopped(const FixSocketAcceptor_t *obj);
void FixSocketAcceptor_delete(const FixSocketAcceptor_t *obj);

FixSocketInitiator_t *FixSocketInitiator_new(FixApplication_t *application, FixMessageStoreFactory_t *storeFactory,
                                             const FixSessionSettings_t *settings, FixLogFactory_t *logFactory);
int8_t FixSocketInitiator_start(FixSocketInitiator_t *obj);
int8_t FixSocketInitiator_block(FixSocketInitiator_t *obj);
int8_t FixSocketInitiator_poll(FixSocketInitiator_t *obj);
int8_t FixSocketInitiator_stop(FixSocketInitiator_t *obj);
int8_t FixSocketInitiator_isLoggedOn(const FixSocketInitiator_t *obj);
int8_t FixSocketInitiator_isStopped(const FixSocketInitiator_t *obj);
void FixSocketInitiator_delete(const FixSocketInitiator_t *obj);

FixSessionID_t *FixSessionID_new(const char *beginString, const char *senderCompID, const char *targetCompID,
                                 const char *sessionQualifier);
FixSessionID_t *FixSessionID_copy(const FixSessionID_t *src);
const char *FixSessionID_getBeginString(const FixSessionID_t *session);
const char *FixSessionID_getSenderCompID(const FixSessionID_t *session);
const char *FixSessionID_getTargetCompID(const FixSessionID_t *session);
const char *FixSessionID_getSessionQualifier(const FixSessionID_t *session);
int8_t FixSessionID_isFIXT(const FixSessionID_t *session);
const char *FixSessionID_toString(const FixSessionID_t *session);
void FixSessionID_delete(const FixSessionID_t *session);

FixMessage_t *FixMessage_new();
FixMessage_t *FixMessage_copy(const FixMessage_t *src);
FixMessage_t *FixMessage_fromString(const char *text);
FixMessage_t *FixMessage_fromStringAndDictionary(const char *text, const FixDataDictionary_t *dictionary);
const char *FixMessage_getField(const FixMessage_t *obj, int32_t tag);
int8_t FixMessage_setField(FixMessage_t *obj, int32_t tag, const char *value);
int8_t FixMessage_removeField(FixMessage_t *obj, int32_t tag);
int8_t FixMessage_addGroup(FixMessage_t *obj, const FixGroup_t *group);
int64_t FixMessage_getStringLen(const FixMessage_t *obj);
int8_t FixMessage_readString(const FixMessage_t *obj, char *buffer, uint64_t buffer_len);
void FixMessage_delete(const FixMessage_t *obj);

FixHeader_t *FixHeader_new();
FixHeader_t *FixHeader_copy(const FixHeader_t *src);
FixHeader_t *FixMessage_copyHeader(const FixMessage_t *obj);
FixHeader_t *FixMessage_getHeaderRef(FixMessage_t *obj);
const char *FixHeader_getField(const FixHeader_t *obj, int32_t tag);
int8_t FixHeader_setField(FixHeader_t *obj, int32_t tag, const char *value);
int8_t FixHeader_removeField(FixHeader_t *obj, int32_t tag);
int8_t FixHeader_addGroup(FixHeader_t *obj, const FixGroup_t *group);
void FixHeader_delete(const FixHeader_t *obj);

FixTrailer_t *FixTrailer_new();
FixTrailer_t *FixTrailer_copy(const FixTrailer_t *src);
FixTrailer_t *FixMessage_copyTrailer(const FixMessage_t *obj);
FixTrailer_t *FixMessage_getTrailerRef(FixMessage_t *obj);
const char *FixTrailer_getField(const FixTrailer_t *obj, int32_t tag);
int8_t FixTrailer_setField(FixTrailer_t *obj, int32_t tag, const char *value);
int8_t FixTrailer_removeField(FixTrailer_t *obj, int32_t tag);
int8_t FixTrailer_addGroup(FixTrailer_t *obj, const FixGroup_t *group);
void FixTrailer_delete(const FixTrailer_t *obj);

FixGroup_t *FixGroup_new(int32_t fieldId, int32_t delim, const int32_t order[]);
FixGroup_t *FixGroup_copy(const FixGroup_t *src);
FixGroup_t *FixMessage_copyGroup(const FixMessage_t *obj, int32_t num, int32_t tag);
FixGroup_t *FixHeader_copyGroup(const FixHeader_t *obj, int32_t num, int32_t tag);
FixGroup_t *FixTrailer_copyGroup(const FixTrailer_t *obj, int32_t num, int32_t tag);
FixGroup_t *FixGroup_copyGroup(const FixGroup_t *obj, int32_t num, int32_t tag);
FixGroup_t *FixMessage_getGroupRef(const FixMessage_t *obj, int32_t num, int32_t tag);
int32_t FixGroup_getFieldId(const FixGroup_t *obj);
int32_t FixGroup_getDelim(const FixGroup_t *obj);
const char *FixGroup_getField(const FixGroup_t *obj, int32_t tag);
int8_t FixGroup_setField(FixGroup_t *obj, int32_t tag, const char *value);
int8_t FixGroup_removeField(FixGroup_t *obj, int32_t tag);
int8_t FixGroup_addGroup(FixGroup_t *obj, const FixGroup_t *group);
void FixGroup_delete(const FixGroup_t *obj);

int8_t FixSession_sendToTarget(FixMessage_t *msg, const FixSessionID_t *session_id);
FixSession_t *FixSession_lookup(const FixSessionID_t *session_id);
int8_t FixSession_logout(FixSession_t *session);
int8_t FixSession_isLoggedOn(FixSession_t *session);
int8_t FixSession_send(FixSession_t *session, FixMessage_t *msg);

#ifdef __cplusplus
}
}
#endif // __cplusplus

#endif // _QUICKFIX_BIND_H_
