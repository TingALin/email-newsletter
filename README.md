# Ting's newsletter

## Goal
* To achieve high-availability while running in fault-prone environments
* To allow us to continuously release new versions with zero downtime
* To handle dynamic workloads (e.g. request volumes)

## Project skeleton
Faster Linking: The default linker does a good job, but there is a faster alternative: lld, a linker developed by the LLVM projects

## Tech stacks
Continuous Integration (CI) pipeline
* actix-web: a web framework(it runs on tokio, therefore minimising the likelihood of having to deal with incompatibilities and interop between different async runtimes) 
* pingdom.com:  a SaaS service to combine it with and you can be alerted when your API goes dark
* Kubernetes: A health-check endpoint can also be handy if you are using a container orchestrator to juggle your application: the orchestrator can call /health_check to detect if the API has become unresponsive and trigger a restart for the load balancing and ensure availability

### Inner development loop
Make a change
Compile the application
Run tests
Run the application

### Todos
github workflow
API development
UI
tests: https://actix.rs/docs/testing/

### Roadmap

### Alternative
* bastion’s actor framework: implement our own asynchronous runtime: https://github.com/bastion-rs/bastion


### Our Strategy
choose a web framework and get familiar with it;
define our testing strategy;
choose a crate to interact with our database (we will have to save those emails somewhere!);
define how we want to manage changes to our database schemas over time (a.k.a. migrations);
actually write some queries