# Vue Rust Todo App


## Server Deploy
Deployed to Heroku using this buildpack: https://github.com/emk/heroku-buildpack-rust

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
