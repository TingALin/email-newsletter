# Rust Newsletter
An cloud-native email newsletter application in Rust

## Our Goal
* To achieve high-availability while running in fault-prone environments
* To allow us to continuously release new versions with zero downtime
* To handle dynamic workloads (e.g. request volumes)

### Workflow
Our blog visitors input their email address in a form embedded on a web page.
The form will trigger a POST /subscriptions call to our backend API that will actually process the information, store it and send back a response

### Tech Stacks/Tools
* actix-web: a web framework(it runs on tokio, therefore minimising the likelihood of having to deal with incompatibilities and interop between different async runtimes) 
* pingdom.com:  a SaaS service to combine it with and you can be alerted when your API goes dark
* Kubernetes: A health-check endpoint can also be handy if you are using a container orchestrator to juggle your application: the orchestrator can call /health_check to detect if the API has become unresponsive and trigger a restart for the load balancing and ensure availability
* Continuous Integration (CI) pipeline
