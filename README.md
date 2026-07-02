# IAM

# _This platform can be used by other system for there Authentication and Authorization Layer_

So instead of creating the auth and authorization layer form scratch they can use this platform to plug it in there system and can use the auth and authorization easily 

In you Organization you can create custom roles and assingn different permission to different roles (e.g endusers get different permission form the Owner and Admin)

So it have Three layers
- **Layer 1** -> Authentication : Which handels the login,logout,session and token refresh
- **Layer 2** -> Organization : Which handles different comapnies from each other in a same system
- **Layer 3** -> RBAC : Which handels what a specific person is allowed to do and have access to


You can See the Architecture and Desgin here DESIGN.md


# The is the DB Relation
<img width="1000" src="https://github.com/user-attachments/assets/fba6d491-2ddc-44b2-9c3e-001b00648cc4" />


