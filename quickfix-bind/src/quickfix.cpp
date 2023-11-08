#include "quickfix_bind.h"

#include <quickfix/FileStore.h>
#include <quickfix/FileLog.h>
#include <quickfix/SocketAcceptor.h>
#include <quickfix/Session.h>
#include <quickfix/SessionID.h>
#include <quickfix/SessionSettings.h>
#include <quickfix/Application.h>
#include <quickfix/Message.h>

#ifdef __cplusplus
extern "C"
{
#endif

    class ApplicationBind : public FIX::Application
    {
    private:
        FixApplicationCallbacks_t *callbacks;

    public:
        ApplicationBind(FixApplicationCallbacks_t *callbacks)
            : callbacks(callbacks)
        {
        }

        virtual ~ApplicationBind()
        {
        }

        void onCreate(const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->onCreate((FixSessionID_t *)(&session));
        }

        void onLogon(const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->onLogon((FixSessionID_t *)(&session));
        }

        void onLogout(const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->onLogout((FixSessionID_t *)(&session));
        }

        void toAdmin(FIX::Message &msg, const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->toAdmin((FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void toApp(FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::DoNotSend) override
        {
            assert(callbacks != nullptr);
            callbacks->toApp((FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void fromAdmin(const FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue, FIX::RejectLogon) override
        {
            assert(callbacks != nullptr);
            callbacks->fromAdmin((FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void fromApp(const FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue, FIX::UnsupportedMessageType) override
        {
            assert(callbacks != nullptr);
            callbacks->fromApp((FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }
    };

    FixSessionSettings_t *
    FixSessionSettings_new(const char *configPath)
    {
        return (FixSessionSettings_t *)(new FIX::SessionSettings(configPath));
    }

    void FixSessionSettings_delete(FixSessionSettings_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::SessionSettings *)(obj);
        delete fix_obj;
    }

    FixFileStoreFactory_t *
    FixFileStoreFactory_new(FixSessionSettings_t *settings)
    {
        if (settings == nullptr)
            return NULL;

        auto fix_settings = *(FIX::SessionSettings *)(settings);
        return (FixFileStoreFactory_t *)(new FIX::FileStoreFactory(fix_settings));
    }

    void FixFileStoreFactory_delete(FixFileStoreFactory_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::FileStoreFactory *)(obj);
        delete fix_obj;
    }

    FixFileLogFactory_t *
    FixFileLogFactory_new(FixSessionSettings_t *settings)
    {
        if (settings == nullptr)
            return NULL;

        auto fix_settings = *(FIX::SessionSettings *)(settings);
        return (FixFileLogFactory_t *)(new FIX::FileLogFactory(fix_settings));
    }

    void FixFileLogFactory_delete(FixFileLogFactory_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::FileLogFactory *)(obj);
        delete fix_obj;
    }

    FixApplication_t *FixApplication_new(FixApplicationCallbacks_t *callbacks)
    {
        if (callbacks == nullptr)
            return NULL;

        return (FixApplication_t *)(new ApplicationBind(callbacks));
    }

    void FixApplication_delete(FixApplication_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (ApplicationBind *)(obj);
        delete fix_obj;
    }

    FixSocketAcceptor_t *FixSocketAcceptor_new(FixApplication_t *application, FixFileStoreFactory_t *storeFactory, FixSessionSettings_t *settings, FixFileLogFactory_t *logFactory)
    {
        if (application == nullptr || storeFactory == nullptr || logFactory == nullptr || settings == nullptr)
            return NULL;

        auto fix_application = *(ApplicationBind *)(application);
        auto fix_store_factory = *(FIX::FileStoreFactory *)(storeFactory);
        auto fix_log_factory = *(FIX::FileLogFactory *)(logFactory);
        auto fix_settings = *(FIX::SessionSettings *)(settings);

        return (FixSocketAcceptor_t *)(new FIX::SocketAcceptor(fix_application, fix_store_factory, fix_settings, fix_log_factory));
    }

    void FixSocketAcceptor_start(FixSocketAcceptor_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::SocketAcceptor *)(obj);
        fix_obj->start();
    }

    void FixSocketAcceptor_stop(FixSocketAcceptor_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::SocketAcceptor *)(obj);
        fix_obj->stop();
    }

    void FixSocketAcceptor_delete(FixSocketAcceptor_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::SocketAcceptor *)(obj);
        delete fix_obj;
    }

#ifdef __cplusplus
}
#endif // __cplusplus
