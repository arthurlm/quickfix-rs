#ifndef _QUICKFIX_BIND_H_
#define _QUICKFIX_BIND_H_

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct FixSessionSettings FixSessionSettings_t;
    typedef struct FixFileStoreFactory FixFileStoreFactory_t;
    typedef struct FixFileLogFactory FixFileLogFactory_t;
    typedef struct FixApplication FixApplication_t;
    typedef struct FixSocketAcceptor FixSocketAcceptor_t;
    typedef struct FixSessionID FixSessionID_t;
    typedef struct FixMessage FixMessage_t;

    typedef struct FixApplicationCallbacks
    {
        void (*onCreate)(const FixSessionID_t *session);
        void (*onLogon)(const FixSessionID_t *session);
        void (*onLogout)(const FixSessionID_t *session);
        void (*toAdmin)(FixMessage_t *msg, const FixSessionID_t *session);
        void (*toApp)(FixMessage_t *msg, const FixSessionID_t *session);
        void (*fromAdmin)(const FixMessage_t *msg, const FixSessionID_t *session);
        void (*fromApp)(const FixMessage_t *msg, const FixSessionID_t *session);
    } FixApplicationCallbacks_t;

    FixSessionSettings_t *FixSessionSettings_new(const char *configPath);
    void FixSessionSettings_delete(FixSessionSettings_t *obj);

    FixFileStoreFactory_t *FixFileStoreFactory_new(FixSessionSettings_t *settings);
    void FixFileStoreFactory_delete(FixFileStoreFactory_t *obj);

    FixFileLogFactory_t *FixFileLogFactory_new(FixSessionSettings_t *settings);
    void FixFileLogFactory_delete(FixFileLogFactory_t *obj);

    FixApplication_t *FixApplication_new(FixApplicationCallbacks_t *callbacks);
    void FixApplication_delete(FixApplication_t *obj);

    FixSocketAcceptor_t *FixSocketAcceptor_new(FixApplication_t *application, FixFileStoreFactory_t *storeFactory, FixSessionSettings_t *settings, FixFileLogFactory_t *logFactory);
    int FixSocketAcceptor_start(FixSocketAcceptor_t *obj);
    int FixSocketAcceptor_stop(FixSocketAcceptor_t *obj);
    void FixSocketAcceptor_delete(FixSocketAcceptor_t *obj);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _QUICKFIX_BIND_H_
