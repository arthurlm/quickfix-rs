#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

#include "quickfix_bind.h"

static void customOnCreate(const void *data, const FixSessionID_t *session)
{
    printf("customOnCreate: %p %p\n", data, session);
}

static void customOnLogon(const void *data, const FixSessionID_t *session)
{
    printf("customOnLogon: %p %p\n", data, session);
}

static void customOnLogout(const void *data, const FixSessionID_t *session)
{
    printf("customOnLogout: %p %p\n", data, session);
}

static void customToAdmin(const void *data, const FixMessage_t *msg, const FixSessionID_t *session)
{
    printf("customToAdmin: %p %p %p\n", data, msg, session);
}

static void customToApp(const void *data, const FixMessage_t *msg, const FixSessionID_t *session)
{
    printf("customToApp: %p %p %p\n", data, msg, session);
}

static void customFromAdmin(const void *data, const FixMessage_t *msg, const FixSessionID_t *session)
{
    printf("customFromAdmin: %p %p %p\n", data, msg, session);
}

static void customFromApp(const void *data, const FixMessage_t *msg, const FixSessionID_t *session)
{
    printf("customFromApp: %p %p %p\n", data, msg, session);
}

int main(int argc, char **argv)
{
    if (argc != 2)
    {
        fprintf(stderr, "Bad program usage: %s <config_file>\n", argv[0]);
        exit(1);
    }

    const FixApplicationCallbacks_t callbacks = {
        .onCreate = customOnCreate,
        .onLogon = customOnLogon,
        .onLogout = customOnLogout,
        .toAdmin = customToAdmin,
        .toApp = customToApp,
        .fromAdmin = customFromAdmin,
        .fromApp = customFromApp,
    };

    printf(">> Creating resources\n");
    FixSessionSettings_t *settings = FixSessionSettings_new(argv[1]);
    FixFileStoreFactory_t *storeFactory = FixFileStoreFactory_new(settings);
    FixFileLogFactory_t *logFactory = FixFileLogFactory_new(settings);
    FixApplication_t *application = FixApplication_new((void *)0xBEEF, &callbacks);
    FixSocketAcceptor_t *acceptor = FixSocketAcceptor_new(application, storeFactory, settings, logFactory);

    printf(">> Acceptor START\n");
    FixSocketAcceptor_start(acceptor);

    printf(">> Press Q to exit\n");
    while (getchar() != 'q')
    {
    }

    printf(">> Acceptor STOP\n");
    FixSocketAcceptor_stop(acceptor);

    printf(">> Cleaning resources\n");
    FixSocketAcceptor_delete(acceptor);
    FixApplication_delete(application);
    FixFileLogFactory_delete(logFactory);
    FixFileStoreFactory_delete(storeFactory);
    FixSessionSettings_delete(settings);

    return 0;
}
