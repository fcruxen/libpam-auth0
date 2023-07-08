# Libpam-Auth0

This small piece of code is a pam module that authenticates against auth0.

The example in this repo is allowing  No Machine and ssh access using this module, the conf folder 
contains the ```pam.d``` configurations for this.


## Build, Run and Test

Change the values for the constants in ```src/lib.rs```, lines 16~17. 

    const AUTH0_TOKEN_URL: &str = "...";
    const AUTH0_CLIENT_ID: &str = "...";    

Then build the library:

    # cargo build

Now build the docker image:

    # docker-compose build

Run and test:

    # docker-compose up

Now you can connect to ```localhost``` on port ```4000``` using No Machine and login using a username and password defined in your auth0 account.