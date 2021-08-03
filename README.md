# Vue Rust Counter App


## Server Deploy
First make sure you allow the port to be set by an environment variable (which will be set by Heroku)
```rs
use std::env
...
let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
let server_addr = format!("0.0.0.0:{}", port);
```

Deploy to Heroku using this buildpack: https://github.com/emk/heroku-buildpack-rust
```
heroku create --buildpack emk/rust
```

Make a file in ```server/``` called ```Procfile``` with contents
```
web: ./target/release/server
```

### Deploy a subdirectory to heroku
```
git subtree push --prefix server heroku main
```

### Heroku commands
```
heroku git:remote -a <app-name> #set remote app
heroku run bash -a <app-name> #ssh into remote app
heroku config:set YOUR_ENV_VARIABLE=value #set environment variable
```

## Misc
- Heroku probably deploys to AWS by default, using Elastic Beanstalk as a load balancer. Pinging is therefore necessary to keep the WebSocket connections alive, otherwise AWS EB times out after two minutes or so.

## License
MIT License
