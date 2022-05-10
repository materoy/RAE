# RAE
RAE (Remote Application Execution), execute your local applications remotely 

This is just my test for the ultimate CS problem of truly distributed.

At the moment this program only runs a single stdout program remotely - not distributed thought
we'll have to wait for that.


###### Only works for unix systems. Windows user will have to wait a bit


To get Started 

1. Install rust 
2. Clone the repo `git clone https://github.com/rmgen/RAE.git`
3. `cd RAE`
4. Run server `cargo run . --server true` - can be run locally or on a remote machine. I have provisioned for both cases
5. Run program remotely `cargo run . --program [PATH_TO_PROGRAM_EXECUTABLE] --host [REMOTE_HOST_ADDRESS]`
