# Dockerized Lotto

[Rust on Dockerhub](https://hub.docker.com/_/rust)

You have to log in to push to DockerHub:

-   docker login -u mjwoptimizely

After that you build, you tag, you push:

-   docker build -t lotto .
-   docker tag f9f186dfda mjwoptimizely/lotto:2.0
-   docker push mjwoptimizely/lotto:2.0
-   docker tag f9f186dfda mjwoptimizely/lotto:latest
-   docker push mjwoptimizely/lotto:latest

Where that third parameter to `docker tag` is an image ID.

Then you can run it something like this:

-   docker run mjwoptimizely/lotto
-   docker run mjwoptimizely/lotto:2.0

Even if you've deleted the Docker image locally. So that's nice.
