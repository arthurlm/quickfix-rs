#ifndef _QUICKFIX_BIND_H_
#define _QUICKFIX_BIND_H_

#include <stdint.h>
#include <stddef.h>

#ifndef PRINT_QUICKFIX_EX_STDOUT
#define PRINT_QUICKFIX_EX_STDOUT 0
#endif // PRINT_QUICKFIX_EX_STDOUT

#define ERRNO_INVAL -1
#define ERRNO_EXCEPTION -2
#define ERRNO_BUFFER_TO_SMALL -3

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct FixSessionSettings FixSessionSettings_t;
    typedef struct FixDataDictionary FixDataDictionary_t;
    typedef struct FixFileStoreFactory FixFileStoreFactory_t;
    typedef struct FixFileLogFactory FixFileLogFactory_t;
    typedef struct FixApplication FixApplication_t;
    typedef struct FixSocketAcceptor FixSocketAcceptor_t;
    typedef struct FixSocketInitiator FixSocketInitiator_t;
    typedef struct FixSessionID FixSessionID_t;
    typedef struct FixMessage FixMessage_t;
    typedef struct FixHeader FixHeader_t;
    typedef struct FixTrailer FixTrailer_t;
    typedef struct FixGroup FixGroup_t;

    typedef struct FixApplicationCallbacks
    {
        void (*onCreate)(const void *data, const FixSessionID_t *session);
        void (*onLogon)(const void *data, const FixSessionID_t *session);
        void (*onLogout)(const void *data, const FixSessionID_t *session);
        void (*toAdmin)(const void *data, const FixMessage_t *msg, const FixSessionID_t *session);
        void (*toApp)(const void *data, const FixMessage_t *msg, const FixSessionID_t *session);
        void (*fromAdmin)(const void *data, const FixMessage_t *msg, const FixSessionID_t *session);
        void (*fromApp)(const void *data, const FixMessage_t *msg, const FixSessionID_t *session);
    } FixApplicationCallbacks_t;

    FixSessionSettings_t *FixSessionSettings_new();
    FixSessionSettings_t *FixSessionSettings_fromPath(const char *configPath);
    void FixSessionSettings_delete(FixSessionSettings_t *obj);

    FixDataDictionary_t *FixDataDictionary_new();
    FixDataDictionary_t *FixDataDictionary_fromPath(const char *configPath);
    void FixDataDictionary_delete(FixDataDictionary_t *obj);

    FixFileStoreFactory_t *FixFileStoreFactory_new(const FixSessionSettings_t *settings);
    void FixFileStoreFactory_delete(FixFileStoreFactory_t *obj);

    FixFileLogFactory_t *FixFileLogFactory_new(const FixSessionSettings_t *settings);
    void FixFileLogFactory_delete(FixFileLogFactory_t *obj);

    FixApplication_t *FixApplication_new(const void *data, const FixApplicationCallbacks_t *callbacks);
    void FixApplication_delete(FixApplication_t *obj);

    FixSocketAcceptor_t *FixSocketAcceptor_new(const FixApplication_t *application, const FixFileStoreFactory_t *storeFactory, const FixSessionSettings_t *settings, const FixFileLogFactory_t *logFactory);
    int8_t FixSocketAcceptor_start(const FixSocketAcceptor_t *obj);
    int8_t FixSocketAcceptor_block(const FixSocketAcceptor_t *obj);
    int8_t FixSocketAcceptor_poll(const FixSocketAcceptor_t *obj);
    int8_t FixSocketAcceptor_stop(const FixSocketAcceptor_t *obj);
    int8_t FixSocketAcceptor_isLoggedOn(const FixSocketAcceptor_t *obj);
    int8_t FixSocketAcceptor_isStopped(const FixSocketAcceptor_t *obj);
    void FixSocketAcceptor_delete(FixSocketAcceptor_t *obj);

    FixSocketInitiator_t *FixSocketInitiator_new(const FixApplication_t *application, const FixFileStoreFactory_t *storeFactory, const FixSessionSettings_t *settings, const FixFileLogFactory_t *logFactory);
    int8_t FixSocketInitiator_start(const FixSocketInitiator_t *obj);
    int8_t FixSocketInitiator_block(const FixSocketInitiator_t *obj);
    int8_t FixSocketInitiator_poll(const FixSocketInitiator_t *obj);
    int8_t FixSocketInitiator_stop(const FixSocketInitiator_t *obj);
    int8_t FixSocketInitiator_isLoggedOn(const FixSocketInitiator_t *obj);
    int8_t FixSocketInitiator_isStopped(const FixSocketInitiator_t *obj);
    void FixSocketInitiator_delete(FixSocketInitiator_t *obj);

    FixSessionID_t *FixSessionID_new(const char *beginString, const char *senderCompID, const char *targetCompID, const char *sessionQualifier);
    FixSessionID_t *FixSessionID_copy(const FixSessionID_t *src);
    const char *FixSessionID_getBeginString(const FixSessionID_t *session);
    const char *FixSessionID_getSenderCompID(const FixSessionID_t *session);
    const char *FixSessionID_getTargetCompID(const FixSessionID_t *session);
    const char *FixSessionID_getSessionQualifier(const FixSessionID_t *session);
    int8_t FixSessionID_isFIXT(const FixSessionID_t *session);
    const char *FixSessionID_toString(const FixSessionID_t *session);
    void FixSessionID_delete(FixSessionID_t *session);

    FixMessage_t *FixMessage_new();
    FixMessage_t *FixMessage_fromString(const char *text);
    FixMessage_t *FixMessage_fromStringAndDictionary(const char *text, const FixDataDictionary_t *dictionary);
    const char *FixMessage_getField(const FixMessage_t *obj, int32_t tag);
    int8_t FixMessage_setField(const FixMessage_t *obj, int32_t tag, const char *value);
    int8_t FixMessage_removeField(const FixMessage_t *obj, int32_t tag);
    int8_t FixMessage_toBuffer(const FixMessage_t *obj, char *buffer, size_t length);
    void FixMessage_delete(FixMessage_t *obj);

    FixHeader_t *FixMessage_copyHeader(const FixMessage_t *obj);
    FixHeader_t *FixMessage_getHeaderRef(const FixMessage_t *obj);
    const char *FixHeader_getField(const FixHeader_t *obj, int32_t tag);
    int8_t FixHeader_setField(const FixHeader_t *obj, int32_t tag, const char *value);
    int8_t FixHeader_removeField(const FixHeader_t *obj, int32_t tag);
    void FixHeader_delete(FixHeader_t *obj);

    FixTrailer_t *FixMessage_copyTrailer(const FixMessage_t *obj);
    FixTrailer_t *FixMessage_getTrailerRef(const FixMessage_t *obj);
    const char *FixTrailer_getField(const FixTrailer_t *obj, int32_t tag);
    int8_t FixTrailer_setField(const FixTrailer_t *obj, int32_t tag, const char *value);
    int8_t FixTrailer_removeField(const FixTrailer_t *obj, int32_t tag);
    void FixTrailer_delete(FixTrailer_t *obj);

    FixGroup_t *FixMessage_copyGroup(const FixMessage_t *obj, int32_t num, int32_t tag);
    FixGroup_t *FixMessage_getGroupRef(const FixMessage_t *obj, int32_t num, int32_t tag);
    const char *FixGroup_getField(const FixGroup_t *obj, int32_t tag);
    int8_t FixGroup_setField(const FixGroup_t *obj, int32_t tag, const char *value);
    int8_t FixGroup_removeField(const FixGroup_t *obj, int32_t tag);
    void FixGroup_delete(FixGroup_t *obj);

    int8_t FixSession_sendToTarget(const FixMessage_t *msg, const FixSessionID_t *session_id);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _QUICKFIX_BIND_H_
