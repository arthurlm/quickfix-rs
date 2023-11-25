#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "quickfix_bind.h"

static void customOnCreate(const void *data, const FixSessionID_t *session) {
  printf("customOnCreate: %p %p\n", data, session);
}

static void customOnLogon(const void *data, const FixSessionID_t *session) {
  printf("customOnLogon: %p %p\n", data, session);
}

static void customOnLogout(const void *data, const FixSessionID_t *session) {
  printf("customOnLogout: %p %p\n", data, session);
}

static void customToAdmin(const void *data, const FixMessage_t *msg, const FixSessionID_t *session) {
  printf("customToAdmin: %p %p %p\n", data, msg, session);
}

static void customToApp(const void *data, const FixMessage_t *msg, const FixSessionID_t *session) {
  printf("customToApp: %p %p %p\n", data, msg, session);
}

static void customFromAdmin(const void *data, const FixMessage_t *msg, const FixSessionID_t *session) {
  printf("customFromAdmin: %p %p %p\n", data, msg, session);
}

static void customFromApp(const void *data, const FixMessage_t *msg, const FixSessionID_t *session) {
  printf("customFromApp: %p %p %p\n", data, msg, session);
}

static const FixApplicationCallbacks_t APP_CALLBACKS = {
    .onCreate = customOnCreate,
    .onLogon = customOnLogon,
    .onLogout = customOnLogout,
    .toAdmin = customToAdmin,
    .toApp = customToApp,
    .fromAdmin = customFromAdmin,
    .fromApp = customFromApp,
};

static void customOnIncoming(const void *data, const FixSessionID_t *sessionId, const char *msg) {
  printf("customOnIncoming: %p %p: %s\n", data, sessionId, msg);
}

static void customOnOutgoing(const void *data, const FixSessionID_t *sessionId, const char *msg) {
  printf("customOnOutgoing: %p %p: %s\n", data, sessionId, msg);
}

static void customOnEvent(const void *data, const FixSessionID_t *sessionId, const char *msg) {
  printf("customOnEvent: %p %p: %s\n", data, sessionId, msg);
}

static const FixLogCallbacks_t LOG_CALLBACKS = {
    .onIncoming = customOnIncoming,
    .onOutgoing = customOnOutgoing,
    .onEvent = customOnEvent,
};

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "Bad program usage: %s <config_file>\n", argv[0]);
    exit(1);
  }

  printf(">> Creating resources\n");
  FixSessionSettings_t *settings = FixSessionSettings_fromPath(argv[1]);
  FixMessageStoreFactory_t *storeFactory = FixFileMessageStoreFactory_new(settings);
  FixLogFactory_t *logFactory = FixLogFactory_new((void *)0xFEED, &LOG_CALLBACKS);
  FixApplication_t *application = FixApplication_new((void *)0xBEEF, &APP_CALLBACKS);
  FixSocketAcceptor_t *acceptor = FixSocketAcceptor_new(application, storeFactory, settings, logFactory);

  printf(">> Acceptor START\n");
  FixSocketAcceptor_start(acceptor);

  printf(">> Press Q to exit\n");
  while (getchar() != 'q') {
  }

  printf(">> Acceptor STOP\n");
  FixSocketAcceptor_stop(acceptor);

  printf(">> Cleaning resources\n");
  FixSocketAcceptor_delete(acceptor);
  FixApplication_delete(application);
  FixLogFactory_delete(logFactory);
  FixMessageStoreFactory_delete(storeFactory);
  FixSessionSettings_delete(settings);

  return 0;
}
