# Dockerized Lotto

[Rust on Dockerhub](https://hub.docker.com/_/rust)

You have to log in to push to DockerHub:

-   docker login -u mjwoptimizely

After that you build, you tag, you push:

-   docker build -t lotto .
-   docker tag f9f186dfda6f0b8385841afd7dcf09e3b86264f6b52fcfc96ca331d90ad7861a mjwoptimizely/lotto:2.0
-   docker push mjwoptimizely/lotto:2.0

Where that third parameter to `docker tag` is an image ID.

Then you can run it something like this:

-   docker run mjwoptimizely/lotto
-   docker run mjwoptimizely/lotto:2.0

Even if you've deleted the Docker image locally. So that's nice.
