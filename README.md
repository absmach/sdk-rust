# Rust API client for openapi

This is the SuperMQ Combined Service based on the OpenAPI 3.0 specification.  It is the HTTP API for managing SuperMQ. You can now help us improve the API whether it's by making changes to the definition itself or to the code.
Some useful links:
- [The SuperMQ repository](https://github.com/absmach/supermq)



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.15.1
- Package version: 0.15.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:9001*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CertsApi* | [**create_cert**](Models.md#create_cert) | **POST** /{domainID}/certs | Creates a certificate for client
*CertsApi* | [**get_cert**](Models.md#get_cert) | **GET** /{domainID}/certs/{certID} | Retrieves a certificate
*CertsApi* | [**get_serials**](Models.md#get_serials) | **GET** /{domainID}/serials/{clientID} | Retrieves certificates' serial IDs
*CertsApi* | [**health_get**](Models.md#health_get) | **GET** /health | Retrieves service health check info.
*CertsApi* | [**revoke_cert**](Models.md#revoke_cert) | **DELETE** /{domainID}/certs/{certID} | Revokes a certificate
*ChannelsApi* | [**connect_clients_and_channels**](Models.md#connect_clients_and_channels) | **POST** /{domainID}/channels/connect | Connects client and channel.
*ChannelsApi* | [**connect_clients_to_channel**](Models.md#connect_clients_to_channel) | **POST** /{domainID}/channels/{chanID}/connect | Connects clients to a channel
*ChannelsApi* | [**create_channel**](Models.md#create_channel) | **POST** /{domainID}/channels | Creates new channel
*ChannelsApi* | [**create_channels**](Models.md#create_channels) | **POST** /{domainID}/channels/bulk | Creates new channels
*ChannelsApi* | [**disable_channel**](Models.md#disable_channel) | **POST** /{domainID}/channels/{chanID}/disable | Disables a channel
*ChannelsApi* | [**disconnect_clients_and_channels**](Models.md#disconnect_clients_and_channels) | **POST** /{domainID}/channels/disconnect | Disconnects client and channel.
*ChannelsApi* | [**disconnect_clients_from_channel**](Models.md#disconnect_clients_from_channel) | **POST** /{domainID}/channels/{chanID}/disconnect | Disconnects clients from a channel
*ChannelsApi* | [**domain_id_channels_chan_id_delete**](Models.md#domain_id_channels_chan_id_delete) | **DELETE** /{domainID}/channels/{chanID} | Delete channel for given channel id.
*ChannelsApi* | [**enable_channel**](Models.md#enable_channel) | **POST** /{domainID}/channels/{chanID}/enable | Enables a channel
*ChannelsApi* | [**get_channel**](Models.md#get_channel) | **GET** /{domainID}/channels/{chanID} | Retrieves channel info.
*ChannelsApi* | [**list_channels**](Models.md#list_channels) | **GET** /{domainID}/channels | Lists channels.
*ChannelsApi* | [**remove_channel_parent_group**](Models.md#remove_channel_parent_group) | **DELETE** /{domainID}/channels/{chanID}/parent | Removes a parent group from a channel.
*ChannelsApi* | [**set_channel_parent_group**](Models.md#set_channel_parent_group) | **POST** /{domainID}/channels/{chanID}/parent | Sets a parent group for a channel
*ChannelsApi* | [**update_channel**](Models.md#update_channel) | **PATCH** /{domainID}/channels/{chanID} | Updates channel data.
*ChannelsApi* | [**update_channel_tags**](Models.md#update_channel_tags) | **PATCH** /{domainID}/channels/{chanID}/tags | Updates channel tags.
*ClientsApi* | [**add_client_role_action**](Models.md#add_client_role_action) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/actions | Adds a role action for a client role.
*ClientsApi* | [**add_client_role_member**](Models.md#add_client_role_member) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/members | Adds a member to a client role.
*ClientsApi* | [**bulk_create_clients**](Models.md#bulk_create_clients) | **POST** /{domainID}/clients/bulk | Bulk provisions new clients
*ClientsApi* | [**create_client**](Models.md#create_client) | **POST** /{domainID}/clients | Adds new client
*ClientsApi* | [**create_client_role**](Models.md#create_client_role) | **POST** /{domainID}/clients/{clientID}/roles | Creates a role for a client
*ClientsApi* | [**delete_all_client_role_actions**](Models.md#delete_all_client_role_actions) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/actions/delete-all | Deletes all role actions for a client role.
*ClientsApi* | [**delete_all_client_role_members**](Models.md#delete_all_client_role_members) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/members/delete-all | Deletes all members from a client role.
*ClientsApi* | [**delete_client_role**](Models.md#delete_client_role) | **DELETE** /{domainID}/clients/{clientID}/roles/{roleName} | Deletes client role.
*ClientsApi* | [**delete_client_role_action**](Models.md#delete_client_role_action) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/actions/delete | Deletes role actions for a client role.
*ClientsApi* | [**delete_client_role_members**](Models.md#delete_client_role_members) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/members/delete | Deletes members from a client role.
*ClientsApi* | [**disable_client**](Models.md#disable_client) | **POST** /{domainID}/clients/{clientID}/disable | Disables a client
*ClientsApi* | [**domain_id_clients_client_id_delete**](Models.md#domain_id_clients_client_id_delete) | **DELETE** /{domainID}/clients/{clientID} | Delete client for a client with the given id.
*ClientsApi* | [**enable_client**](Models.md#enable_client) | **POST** /{domainID}/clients/{clientID}/enable | Enables a client
*ClientsApi* | [**get_client**](Models.md#get_client) | **GET** /{domainID}/clients/{clientID} | Retrieves client info
*ClientsApi* | [**get_client_role**](Models.md#get_client_role) | **GET** /{domainID}/clients/{clientID}/roles/{roleName} | Retrieves client role.
*ClientsApi* | [**list_available_actions**](Models.md#list_available_actions) | **GET** /{domainID}/clients/roles/available-actions | Retrieves available actions.
*ClientsApi* | [**list_client_role_actions**](Models.md#list_client_role_actions) | **GET** /{domainID}/clients/{clientID}/roles/{roleName}/actions | Lists client role actions.
*ClientsApi* | [**list_client_role_members**](Models.md#list_client_role_members) | **GET** /{domainID}/clients/{clientID}/roles/{roleName}/members | Lists client role members.
*ClientsApi* | [**list_client_roles**](Models.md#list_client_roles) | **GET** /{domainID}/clients/{clientID}/roles | Retrieves clients roles.
*ClientsApi* | [**list_clients**](Models.md#list_clients) | **GET** /{domainID}/clients | Retrieves clients
*ClientsApi* | [**list_user_clients**](Models.md#list_user_clients) | **GET** /{domainID}/users/{userID}/clients | List clients asssociated with a user.
*ClientsApi* | [**remove_client_parent_group**](Models.md#remove_client_parent_group) | **DELETE** /{domainID}/clients/{clientID}/parent | Removes a parent group from a client.
*ClientsApi* | [**set_client_parent_group**](Models.md#set_client_parent_group) | **POST** /{domainID}/clients/{clientID}/parent | Sets a parent group for a client
*ClientsApi* | [**update_client**](Models.md#update_client) | **PATCH** /{domainID}/clients/{clientID} | Updates name and metadata of the client.
*ClientsApi* | [**update_client_role**](Models.md#update_client_role) | **PUT** /{domainID}/clients/{clientID}/roles/{roleName} | Updates client role.
*ClientsApi* | [**update_client_secret**](Models.md#update_client_secret) | **PATCH** /{domainID}/clients/{clientID}/secret | Updates Secret of the identified client.
*ClientsApi* | [**update_client_tags**](Models.md#update_client_tags) | **PATCH** /{domainID}/clients/{clientID}/tags | Updates tags the client.
*ConfigsApi* | [**create_config**](Models.md#create_config) | **POST** /{domainID}/clients/configs | Adds new config
*ConfigsApi* | [**get_bootstrap_config**](Models.md#get_bootstrap_config) | **GET** /clients/bootstrap/{externalId} | Retrieves configuration.
*ConfigsApi* | [**get_config**](Models.md#get_config) | **GET** /{domainID}/clients/configs/{configId} | Retrieves config info (with channels).
*ConfigsApi* | [**get_configs**](Models.md#get_configs) | **GET** /{domainID}/clients/configs | Retrieves managed configs
*ConfigsApi* | [**get_secure_bootstrap_config**](Models.md#get_secure_bootstrap_config) | **GET** /clients/bootstrap/secure/{externalId} | Retrieves configuration.
*ConfigsApi* | [**remove_config**](Models.md#remove_config) | **DELETE** /{domainID}/clients/configs/{configId} | Removes a Config
*ConfigsApi* | [**update_config**](Models.md#update_config) | **PUT** /{domainID}/clients/configs/{configId} | Updates config info
*ConfigsApi* | [**update_config_certs**](Models.md#update_config_certs) | **PATCH** /{domainID}/clients/configs/certs/{configId} | Updates certs
*ConfigsApi* | [**update_config_connections**](Models.md#update_config_connections) | **PUT** /{domainID}/clients/configs/connections/{configId} | Updates channels the client is connected to
*ConfigsApi* | [**update_config_state**](Models.md#update_config_state) | **PUT** /{domainID}/clients/state/{configId} | Updates Config state.
*DomainsApi* | [**add_domain_role_action**](Models.md#add_domain_role_action) | **POST** /domains/{domainID}/roles/{roleName}/actions | Adds a role action for a domain role.
*DomainsApi* | [**add_domain_role_member**](Models.md#add_domain_role_member) | **POST** /domains/{domainID}/roles/{roleName}/members | Adds a member to a domain role.
*DomainsApi* | [**create_domain_role**](Models.md#create_domain_role) | **POST** /domains/{domainID}/roles | Creates a role for a domain
*DomainsApi* | [**delete_all_domain_role_actions**](Models.md#delete_all_domain_role_actions) | **POST** /domains/{domainID}/roles/{roleName}/actions/delete-all | Deletes all role actions for a domain role.
*DomainsApi* | [**delete_all_domain_role_members**](Models.md#delete_all_domain_role_members) | **POST** /domains/{domainID}/roles/{roleName}/members/delete-all | Deletes all members from a domain role.
*DomainsApi* | [**delete_domain_role**](Models.md#delete_domain_role) | **DELETE** /domains/{domainID}/roles/{roleName} | Deletes domain role.
*DomainsApi* | [**delete_domain_role_action**](Models.md#delete_domain_role_action) | **POST** /domains/{domainID}/roles/{roleName}/actions/delete | Deletes role actions for a domain role.
*DomainsApi* | [**delete_domain_role_members**](Models.md#delete_domain_role_members) | **POST** /domains/{domainID}/roles/{roleName}/members/delete | Deletes members from a domain role.
*DomainsApi* | [**domains_domain_id_disable_post**](Models.md#domains_domain_id_disable_post) | **POST** /domains/{domainID}/disable | Disable a domain
*DomainsApi* | [**domains_domain_id_enable_post**](Models.md#domains_domain_id_enable_post) | **POST** /domains/{domainID}/enable | Enables a domain
*DomainsApi* | [**domains_domain_id_freeze_post**](Models.md#domains_domain_id_freeze_post) | **POST** /domains/{domainID}/freeze | Freeze a domain
*DomainsApi* | [**domains_domain_id_get**](Models.md#domains_domain_id_get) | **GET** /domains/{domainID} | Retrieves domain information
*DomainsApi* | [**domains_domain_id_patch**](Models.md#domains_domain_id_patch) | **PATCH** /domains/{domainID} | Updates name, metadata, tags and alias of the domain.
*DomainsApi* | [**domains_get**](Models.md#domains_get) | **GET** /domains | Retrieves list of domains.
*DomainsApi* | [**domains_post**](Models.md#domains_post) | **POST** /domains | Adds new domain
*DomainsApi* | [**get_domain_role**](Models.md#get_domain_role) | **GET** /domains/{domainID}/roles/{roleName} | Retrieves domain role.
*DomainsApi* | [**list_available_domain_actions**](Models.md#list_available_domain_actions) | **GET** /domains/roles/available-actions | Retrieves available actions.
*DomainsApi* | [**list_domain_role_actions**](Models.md#list_domain_role_actions) | **GET** /domains/{domainID}/roles/{roleName}/actions | Lists domain role actions.
*DomainsApi* | [**list_domain_role_members**](Models.md#list_domain_role_members) | **GET** /domains/{domainID}/roles/{roleName}/members | Lists domain role members.
*DomainsApi* | [**list_domain_roles**](Models.md#list_domain_roles) | **GET** /domains/{domainID}/roles | Retrieves domains roles.
*DomainsApi* | [**update_domain_role**](Models.md#update_domain_role) | **PUT** /domains/{domainID}/roles/{roleName} | Updates domain role.
*GroupsApi* | [**add_children_groups**](Models.md#add_children_groups) | **POST** /{domainID}/groups/{groupID}/children | Add children groups.
*GroupsApi* | [**add_group_role_action**](Models.md#add_group_role_action) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/actions | Adds a role action for a group role.
*GroupsApi* | [**add_group_role_member**](Models.md#add_group_role_member) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/members | Adds a member to a group role.
*GroupsApi* | [**create_group**](Models.md#create_group) | **POST** /{domainID}/groups | Creates new group
*GroupsApi* | [**create_group_role**](Models.md#create_group_role) | **POST** /{domainID}/groups/{groupID}/roles | Creates a role for a group
*GroupsApi* | [**delete_all_group_role_actions**](Models.md#delete_all_group_role_actions) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/actions/delete-all | Deletes all role actions for a group role.
*GroupsApi* | [**delete_all_group_role_members**](Models.md#delete_all_group_role_members) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/members/delete-all | Deletes all members from a group role.
*GroupsApi* | [**delete_group_role**](Models.md#delete_group_role) | **DELETE** /{domainID}/groups/{groupID}/roles/{roleName} | Deletes group role.
*GroupsApi* | [**delete_group_role_action**](Models.md#delete_group_role_action) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/actions/delete | Deletes role actions for a group role.
*GroupsApi* | [**delete_group_role_members**](Models.md#delete_group_role_members) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/members/delete | Deletes members from a group role.
*GroupsApi* | [**disable_group**](Models.md#disable_group) | **POST** /{domainID}/groups/{groupID}/disable | Disables a group
*GroupsApi* | [**domain_id_groups_group_id_delete**](Models.md#domain_id_groups_group_id_delete) | **DELETE** /{domainID}/groups/{groupID} | Delete group for a group with the given id.
*GroupsApi* | [**enable_group**](Models.md#enable_group) | **POST** /{domainID}/groups/{groupID}/enable | Enables a group
*GroupsApi* | [**get_group**](Models.md#get_group) | **GET** /{domainID}/groups/{groupID} | Gets group info.
*GroupsApi* | [**get_group_role**](Models.md#get_group_role) | **GET** /{domainID}/groups/{groupID}/roles/{roleName} | Retrieves group role.
*GroupsApi* | [**list_available_group_actions**](Models.md#list_available_group_actions) | **GET** /{domainID}/groups/roles/available-actions | Retrieves available actions.
*GroupsApi* | [**list_children_groups**](Models.md#list_children_groups) | **GET** /{domainID}/groups/{groupID}/children | List children of a certain group
*GroupsApi* | [**list_group_hierarchy**](Models.md#list_group_hierarchy) | **GET** /{domainID}/groups/{groupID}/hierarchy | Lists groups hierarchy.
*GroupsApi* | [**list_group_role_actions**](Models.md#list_group_role_actions) | **GET** /{domainID}/groups/{groupID}/roles/{roleName}/actions | Lists group role actions.
*GroupsApi* | [**list_group_role_members**](Models.md#list_group_role_members) | **GET** /{domainID}/groups/{groupID}/roles/{roleName}/members | Lists group role members.
*GroupsApi* | [**list_group_roles**](Models.md#list_group_roles) | **GET** /{domainID}/groups/{groupID}/roles | Retrieves groups roles.
*GroupsApi* | [**list_groups**](Models.md#list_groups) | **GET** /{domainID}/groups | Lists groups.
*GroupsApi* | [**remove_all_children_groups**](Models.md#remove_all_children_groups) | **DELETE** /{domainID}/groups/{groupID}/children/all | Remove all children groups.
*GroupsApi* | [**remove_children_groups**](Models.md#remove_children_groups) | **DELETE** /{domainID}/groups/{groupID}/children | Remove children groups.
*GroupsApi* | [**remove_group_parent_group**](Models.md#remove_group_parent_group) | **DELETE** /{domainID}/groups/{groupID}/parent | Removes a parent group from a group.
*GroupsApi* | [**set_group_parent_group**](Models.md#set_group_parent_group) | **POST** /{domainID}/groups/{groupID}/parent | Sets a parent group for a group.
*GroupsApi* | [**update_group**](Models.md#update_group) | **PUT** /{domainID}/groups/{groupID} | Updates group data.
*GroupsApi* | [**update_group_role**](Models.md#update_group_role) | **PUT** /{domainID}/groups/{groupID}/roles/{roleName} | Updates group role.
*InvitationsApi* | [**accept_invitation**](Models.md#accept_invitation) | **POST** /invitations/accept | Accept invitation
*InvitationsApi* | [**delete_invitation**](Models.md#delete_invitation) | **DELETE** /invitations/{user_id}/{domain_id} | Deletes a specific invitation
*InvitationsApi* | [**get_invitation**](Models.md#get_invitation) | **GET** /invitations/{user_id}/{domain_id} | Retrieves a specific invitation
*InvitationsApi* | [**list_invitations**](Models.md#list_invitations) | **GET** /invitations | List invitations
*InvitationsApi* | [**reject_invitation**](Models.md#reject_invitation) | **POST** /invitations/reject | Reject invitation
*InvitationsApi* | [**send_invitation**](Models.md#send_invitation) | **POST** /invitations | Send invitation
*JournalLogApi* | [**domain_id_journal_entity_type_id_get**](Models.md#domain_id_journal_entity_type_id_get) | **GET** /{domainID}/journal/{entityType}/{id} | List entity journal log
*JournalLogApi* | [**journal_user_user_id_get**](Models.md#journal_user_user_id_get) | **GET** /journal/user/{userID} | List user journal log
*KeysApi* | [**get_key**](Models.md#get_key) | **GET** /keys/{keyID} | Gets API key details.
*KeysApi* | [**issue_key**](Models.md#issue_key) | **POST** /keys | Issue API key
*KeysApi* | [**revoke_key**](Models.md#revoke_key) | **DELETE** /keys/{keyID} | Revoke API key
*MessagesApi* | [**channels_id_messages_post**](Models.md#channels_id_messages_post) | **POST** /channels/{id}/messages | Sends message to the communication channel
*NotifiersApi* | [**create_subscription**](Models.md#create_subscription) | **POST** /subscriptions | Create subscription
*NotifiersApi* | [**list_subscriptions**](Models.md#list_subscriptions) | **GET** /subscriptions | List subscriptions
*NotifiersApi* | [**remove_subscription**](Models.md#remove_subscription) | **DELETE** /subscriptions/{id} | Delete subscription with the provided id
*NotifiersApi* | [**view_subscription**](Models.md#view_subscription) | **GET** /subscriptions/{id} | Get subscription with the provided id
*ProvisionApi* | [**domain_id_mapping_get**](Models.md#domain_id_mapping_get) | **GET** /{domainID}/mapping | Gets current mapping.
*ProvisionApi* | [**domain_id_mapping_post**](Models.md#domain_id_mapping_post) | **POST** /{domainID}/mapping | Adds new device to proxy
*ReadersApi* | [**get_messages**](Models.md#get_messages) | **GET** /channels/{chanId}/messages | Retrieves messages sent to single channel
*StatesApi* | [**get_states**](Models.md#get_states) | **GET** /states/{twinID} | Retrieves states of twin with id twinID
*TwinsApi* | [**create_twin**](Models.md#create_twin) | **POST** /twins | Adds new twin
*TwinsApi* | [**get_twin**](Models.md#get_twin) | **GET** /twins/{twinID} | Retrieves twin info
*TwinsApi* | [**get_twins**](Models.md#get_twins) | **GET** /twins | Retrieves twins
*TwinsApi* | [**remove_twin**](Models.md#remove_twin) | **DELETE** /twins/{twinID} | Removes a twin
*TwinsApi* | [**update_twin**](Models.md#update_twin) | **PUT** /twins/{twinID} | Updates twin info
*UsersApi* | [**create_user**](Models.md#create_user) | **POST** /users | Registers user account
*UsersApi* | [**disable_user**](Models.md#disable_user) | **POST** /users/{userID}/disable | Disables a user
*UsersApi* | [**domain_id_users_get**](Models.md#domain_id_users_get) | **GET** /{domainID}/users | List users assigned to domain
*UsersApi* | [**enable_user**](Models.md#enable_user) | **POST** /users/{userID}/enable | Enables a user
*UsersApi* | [**get_profile**](Models.md#get_profile) | **GET** /users/profile | Gets info on currently logged in user.
*UsersApi* | [**get_user**](Models.md#get_user) | **GET** /users/{userID} | Retrieves a user
*UsersApi* | [**issue_token**](Models.md#issue_token) | **POST** /users/tokens/issue | Issue Token
*UsersApi* | [**list_users**](Models.md#list_users) | **GET** /users | List users
*UsersApi* | [**list_users_in_channel**](Models.md#list_users_in_channel) | **GET** /{domainID}/channels/{channelID}/users | List users in a channel
*UsersApi* | [**list_users_in_client**](Models.md#list_users_in_client) | **GET** /{domainID}/clients/{clientID}/users | List users associated with a client
*UsersApi* | [**list_users_in_group**](Models.md#list_users_in_group) | **GET** /{domainID}/groups/{groupID}/users | List users in a group
*UsersApi* | [**refresh_token**](Models.md#refresh_token) | **POST** /users/tokens/refresh | Refresh Token
*UsersApi* | [**request_password_reset**](Models.md#request_password_reset) | **POST** /password/reset-request | User password reset request
*UsersApi* | [**reset_password**](Models.md#reset_password) | **PUT** /password/reset | User password reset endpoint
*UsersApi* | [**search_users**](Models.md#search_users) | **GET** /users/search | Search users
*UsersApi* | [**update_email**](Models.md#update_email) | **PATCH** /users/{userID}/email | Updates email of the user.
*UsersApi* | [**update_profile_picture**](Models.md#update_profile_picture) | **PATCH** /users/{userID}/picture | Updates the user's profile picture.
*UsersApi* | [**update_role**](Models.md#update_role) | **PATCH** /users/{userID}/role | Updates the user's role.
*UsersApi* | [**update_secret**](Models.md#update_secret) | **PATCH** /users/secret | Updates secret of currently logged in user.
*UsersApi* | [**update_tags**](Models.md#update_tags) | **PATCH** /users/{userID}/tags | Updates tags of the user.
*UsersApi* | [**update_user**](Models.md#update_user) | **PATCH** /users/{userID} | Updates first, last name and metadata of the user.
*UsersApi* | [**update_username**](Models.md#update_username) | **PATCH** /users/{userID}/username | Updates user's username.
*UsersApi* | [**users_user_id_delete**](Models.md#users_user_id_delete) | **DELETE** /users/{userID} | Delete a user


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

info@abstractmachines.fr

