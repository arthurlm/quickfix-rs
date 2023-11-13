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

#define RETURN_IF_NULL(_OBJ_) \
    if ((_OBJ_) == nullptr)   \
        return;

#define RETURN_VAL_IF_NULL(_OBJ_, _VAL_) \
    if ((_OBJ_) == nullptr)              \
        return (_VAL_);

#define DELETE_OBJ(_TYPE_, _OBJ_)         \
    {                                     \
        auto fix_obj = (_TYPE_ *)(_OBJ_); \
        delete fix_obj;                   \
    }

#define RETURN_CXX_TO_C_STR(_TYPE_, _OBJ_, _METHOD_) \
    try                                              \
    {                                                \
        auto fix_obj = (_TYPE_ *)((_OBJ_));          \
        return fix_obj->_METHOD_.c_str();            \
    }                                                \
    catch (std::exception & e)                       \
    {                                                \
        return NULL;                                 \
    }

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

        ApplicationBind(const ApplicationBind &) = delete;
        ApplicationBind &operator=(const ApplicationBind &) = delete;

        virtual ~ApplicationBind()
        {
        }

        void onCreate(const FIX::SessionID &session) override
        {
            RETURN_IF_NULL(callbacks);
            RETURN_IF_NULL(callbacks->onCreate);
            callbacks->onCreate(data, (FixSessionID_t *)(&session));
        }

        void onLogon(const FIX::SessionID &session) override
        {
            RETURN_IF_NULL(callbacks);
            RETURN_IF_NULL(callbacks->onLogon);
            callbacks->onLogon(data, (FixSessionID_t *)(&session));
        }

        void onLogout(const FIX::SessionID &session) override
        {
            RETURN_IF_NULL(callbacks);
            RETURN_IF_NULL(callbacks->onLogout);
            callbacks->onLogout(data, (FixSessionID_t *)(&session));
        }

        void toAdmin(FIX::Message &msg, const FIX::SessionID &session) override
        {
            RETURN_IF_NULL(callbacks);
            RETURN_IF_NULL(callbacks->toAdmin);
            callbacks->toAdmin(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void toApp(FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::DoNotSend) override
        {
            RETURN_IF_NULL(callbacks);
            RETURN_IF_NULL(callbacks->toApp);
            callbacks->toApp(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void fromAdmin(const FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue, FIX::RejectLogon) override
        {
            RETURN_IF_NULL(callbacks);
            RETURN_IF_NULL(callbacks->fromAdmin);
            callbacks->fromAdmin(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }

        void fromApp(const FIX::Message &msg, const FIX::SessionID &session) EXCEPT(FIX::FieldNotFound, FIX::IncorrectDataFormat, FIX::IncorrectTagValue, FIX::UnsupportedMessageType) override
        {
            RETURN_IF_NULL(callbacks);
            RETURN_IF_NULL(callbacks->fromApp);
            callbacks->fromApp(data, (FixMessage_t *)(&msg), (FixSessionID_t *)(&session));
        }
    };

    FixSessionSettings_t *
    FixSessionSettings_fromPath(const char *configPath)
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
        RETURN_IF_NULL(obj);
        DELETE_OBJ(FIX::SessionSettings, obj);
    }

    FixFileStoreFactory_t *
    FixFileStoreFactory_new(const FixSessionSettings_t *settings)
    {
        RETURN_VAL_IF_NULL(settings, NULL);

        auto fix_settings = (FIX::SessionSettings *)(settings);
        try
        {
            return (FixFileStoreFactory_t *)(new FIX::FileStoreFactory(*fix_settings));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    void FixFileStoreFactory_delete(FixFileStoreFactory_t *obj)
    {
        RETURN_IF_NULL(obj);
        DELETE_OBJ(FIX::FileStoreFactory, obj);
    }

    FixFileLogFactory_t *
    FixFileLogFactory_new(const FixSessionSettings_t *settings)
    {
        RETURN_VAL_IF_NULL(settings, NULL);

        auto fix_settings = (FIX::SessionSettings *)(settings);
        try
        {
            return (FixFileLogFactory_t *)(new FIX::FileLogFactory(*fix_settings));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    void FixFileLogFactory_delete(FixFileLogFactory_t *obj)
    {
        RETURN_IF_NULL(obj);
        DELETE_OBJ(FIX::FileLogFactory, obj);
    }

    FixApplication_t *FixApplication_new(const void *data, const FixApplicationCallbacks_t *callbacks)
    {
        RETURN_VAL_IF_NULL(callbacks, NULL);

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
        RETURN_IF_NULL(obj);
        DELETE_OBJ(ApplicationBind, obj);
    }

    FixSocketAcceptor_t *FixSocketAcceptor_new(const FixApplication_t *application, const FixFileStoreFactory_t *storeFactory, const FixSessionSettings_t *settings, const FixFileLogFactory_t *logFactory)
    {
        RETURN_VAL_IF_NULL(application, NULL);
        RETURN_VAL_IF_NULL(storeFactory, NULL);
        RETURN_VAL_IF_NULL(logFactory, NULL);
        RETURN_VAL_IF_NULL(settings, NULL);

        auto fix_application = (ApplicationBind *)(application);
        auto fix_store_factory = (FIX::FileStoreFactory *)(storeFactory);
        auto fix_log_factory = (FIX::FileLogFactory *)(logFactory);
        auto fix_settings = (FIX::SessionSettings *)(settings);

        try
        {
            return (FixSocketAcceptor_t *)(new FIX::SocketAcceptor(*fix_application, *fix_store_factory, *fix_settings, *fix_log_factory));
        }
        catch (std::exception &ex)
        {
            return NULL;
        }
    }

    int FixSocketAcceptor_start(const FixSocketAcceptor_t *obj)
    {
        RETURN_VAL_IF_NULL(obj, -1);

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
        RETURN_VAL_IF_NULL(obj, -1);

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
        RETURN_IF_NULL(obj);
        DELETE_OBJ(FIX::SocketAcceptor, obj);
    }

    const char *FixSessionID_getBeginString(const FixSessionID_t *session)
    {
        RETURN_VAL_IF_NULL(session, NULL);
        RETURN_CXX_TO_C_STR(FIX::SessionID, session, getBeginString().getString())
    }

    const char *FixSessionID_getSenderCompID(const FixSessionID_t *session)
    {
        RETURN_VAL_IF_NULL(session, NULL);
        RETURN_CXX_TO_C_STR(FIX::SessionID, session, getSenderCompID().getString())
    }

    const char *FixSessionID_getTargetCompID(const FixSessionID_t *session)
    {
        RETURN_VAL_IF_NULL(session, NULL);
        RETURN_CXX_TO_C_STR(FIX::SessionID, session, getTargetCompID().getString())
    }

    const char *FixSessionID_getSessionQualifier(const FixSessionID_t *session)
    {
        RETURN_VAL_IF_NULL(session, NULL);
        RETURN_CXX_TO_C_STR(FIX::SessionID, session, getSessionQualifier())
    }

    int8_t FixSessionID_isFIXT(const FixSessionID_t *session)
    {
        RETURN_VAL_IF_NULL(session, 0);

        auto fix_obj = (FIX::SessionID *)(session);
        try
        {
            return fix_obj->isFIXT();
        }
        catch (std::exception &e)
        {
            return 0;
        }
    }

#ifdef __cplusplus
}
#endif // __cplusplus
