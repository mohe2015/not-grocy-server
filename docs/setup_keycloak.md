```bash

# IGNORE BELOW - I NEED TO MOVE THIS SOMEWHERE ELSE

# create realm

# Roles -> Add Role -> add roles for permissions
# stock:view, stock:edit

# Clients -> Create
# access type: confidential
# authorization enabled
# Consent Required
# Scope -> Full Scope Allowed = false
# Assigned Roles: all roles for permissions

# Client Scopes -> add all perms
# for each of them add a "realm roles" mapper???

# Client Scopes -> Default Client Scopes -> Add to optional

# Client Scopes -> for each -> Scope -> Add assigned roles??? I THINK THIS DID IT?

"When issuing tokens for a particular user, the client scope is applied only if the user is permitted to use it. In the case that a client scope does not have any role scope mappings defined on itself, then each user is automatically permitted to use this client scope. However, when a client scope has any role scope mappings defined on itself, then the user must be a member of at least one of the roles. In other words, there must be an intersection between the user roles and the roles of the client scope. Composite roles are taken into account when evaluating this intersection."

# https://github.com/keycloak/keycloak-documentation/blob/master/server_admin/topics/clients/client-scopes.adoc

# Users -> Create
# test
# add role mappings


# Client -> Client Scopes -> Optional Client Scopes -> add all


https://sso.selfmade4u.de/auth/realms/not-grocy/protocol/openid-connect/auth?response_type=code&scope=stock:edit stock:view&client_id=not-grocy&state=CHANGEME&redirect_uri=https://test.selfmade4u.de


## Authorization -> Authorization Scopes -> Create
## stock:view, stock:edit

## Resources -> Create
## stock
## add scopes to resource: stock:view, stock:edit


## Policies:
## Role Policy
## Name: stock:view
## Realm Roles: stock:view
## Maybe required?
## select client


## Permissions:
## Create -> Scope-Based
## perm:stock:view
## resource: stock
## scopes: stock:view
## policy: stock:view


## Client -> Authorization -> Evaluate
## user: test
## roles: stock:view
## resources: stock
## scopes: stock:view


# https://medium.com/@harsh.manvar111/keycloak-authorization-service-rbac-1c3204a33a50

# https://www.keycloak.org/docs/latest/authorization_services/

https://www.keycloak.org/docs/latest/authorization_services/

https://github.com/keycloak/keycloak-quickstarts/tree/latest/app-authz-uma-photoz

User-Managed Access also be interesting

https://sso.selfmade4u.de/auth/realms/not-grocy/account/#/

https://sso.selfmade4u.de/auth/realms/not-grocy/.well-known/openid-configuration

# secret from Client -> Credentials
# TODO FIXME state-parameter
https://sso.selfmade4u.de/auth/realms/not-grocy/protocol/openid-connect/auth?response_type=code&scope=openid%20email&client_id=not-grocy&state=CHANGEME&redirect_uri=https://test.selfmade4u.de


# Keycloak todo find out how to deny claims / scopes


# then get the token from that. as far as I understand this is only possible for the server as you need the client_secret for that

# Realm -> Client Scopes -> Add for all permissions
# clients->client scopes->setup add client scopes to default

# TODO FIXME use client scopes instead of that above

# TODO FIXME keycloak allow user to deny scope

https://www.keycloak.org/app/

# https://lists.jboss.org/pipermail/keycloak-user/2016-September/007621.html
# this may be interesting with realm and client roles?

# https://github.com/keycloak/keycloak-documentation/blob/master/server_admin/topics/clients/client-scopes.adoc

```