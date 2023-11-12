#include "quickfix_bind.h"

#include <exception>

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
        const FixApplicationCallbacks_t *callbacks;
        const void *data;

    public:
        ApplicationBind(const void *data, const FixApplicationCallbacks_t *callbacks)
            : callbacks(callbacks), data(data)
        {
        }

        virtual ~ApplicationBind()
        {
        }

        void onCreate(const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->onCreate(data, (FixSessionID_t *)(&session));
        }

        void onLogon(const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->onLogon(data, (FixSessionID_t *)(&session));
        }

        void onLogout(const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->onLogout(data, (FixSessionID_t *)(&session));
        }

        void toAdmin(FIX::Message &msg, const FIX::SessionID &session) override
        {
            assert(callbacks != nullptr);
            callbacks->toAdmin(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void toApp(FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::DoNotSend) override
        {
            assert(callbacks != nullptr);
            callbacks->toApp(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void fromAdmin(const FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue, FIX::RejectLogon) override
        {
            assert(callbacks != nullptr);
            callbacks->fromAdmin(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void fromApp(const FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue, FIX::UnsupportedMessageType) override
        {
            assert(callbacks != nullptr);
            callbacks->fromApp(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }
    };

    FixSessionSettings_t *
    FixSessionSettings_new(const char *configPath)
    {
        try
        {
            return (FixSessionSettings_t *)(new FIX::SessionSettings(configPath));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    void FixSessionSettings_delete(FixSessionSettings_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::SessionSettings *)(obj);
        delete fix_obj;
    }

    FixFileStoreFactory_t *
    FixFileStoreFactory_new(const FixSessionSettings_t *settings)
    {
        if (settings == nullptr)
            return NULL;

        auto fix_settings = *(FIX::SessionSettings *)(settings);

        try
        {
            return (FixFileStoreFactory_t *)(new FIX::FileStoreFactory(fix_settings));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    void FixFileStoreFactory_delete(FixFileStoreFactory_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::FileStoreFactory *)(obj);
        delete fix_obj;
    }

    FixFileLogFactory_t *
    FixFileLogFactory_new(const FixSessionSettings_t *settings)
    {
        if (settings == nullptr)
            return NULL;

        auto fix_settings = *(FIX::SessionSettings *)(settings);

        try
        {
            return (FixFileLogFactory_t *)(new FIX::FileLogFactory(fix_settings));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    void FixFileLogFactory_delete(FixFileLogFactory_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (FIX::FileLogFactory *)(obj);
        delete fix_obj;
    }

    FixApplication_t *FixApplication_new(const void *data, const FixApplicationCallbacks_t *callbacks)
    {
        if (callbacks == nullptr)
            return NULL;

        try
        {
            return (FixApplication_t *)(new ApplicationBind(data, callbacks));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    void FixApplication_delete(FixApplication_t *obj)
    {
        if (obj == nullptr)
            return;

        auto fix_obj = (ApplicationBind *)(obj);
        delete fix_obj;
    }

    FixSocketAcceptor_t *FixSocketAcceptor_new(const FixApplication_t *application, const FixFileStoreFactory_t *storeFactory, const FixSessionSettings_t *settings, const FixFileLogFactory_t *logFactory)
    {
        if (application == nullptr || storeFactory == nullptr || logFactory == nullptr || settings == nullptr)
            return NULL;

        auto fix_application = *(ApplicationBind *)(application);
        auto fix_store_factory = *(FIX::FileStoreFactory *)(storeFactory);
        auto fix_log_factory = *(FIX::FileLogFactory *)(logFactory);
        auto fix_settings = *(FIX::SessionSettings *)(settings);

        try
        {
            return (FixSocketAcceptor_t *)(new FIX::SocketAcceptor(fix_application, fix_store_factory, fix_settings, fix_log_factory));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    int FixSocketAcceptor_start(const FixSocketAcceptor_t *obj)
    {
        if (obj == nullptr)
            return -1;

        auto fix_obj = (FIX::SocketAcceptor *)(obj);
        try
        {
            fix_obj->start();
        }
        catch (std::exception &ex)
        {
            return -1;
        }
        return 0;
    }

    int FixSocketAcceptor_stop(const FixSocketAcceptor_t *obj)
    {
        if (obj == nullptr)
            return -1;

        auto fix_obj = (FIX::SocketAcceptor *)(obj);
        try
        {
            fix_obj->stop();
        }
        catch (std::exception &ex)
        {
            return -1;
        }
        return 0;
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
