# Dockerized Lotto

[Rust on Dockerhub](https://hub.docker.com/_/rust)

You have to log in to push to DockerHub:

-   docker login -u mjw1200

After that you build, you tag, you push:

-   docker build -t lotto .
-   docker tag f9f186dfda mjw1200/lotto:2.0
-   docker push mjw1200/lotto:2.0

Where that third parameter to `docker tag` is an image ID.

To update the "latest" tag, just tag and push again
-   docker tag f9f186dfda mjw1200/lotto:latest
-   docker push mjw1200/lotto:latest

Then you can run it something like this:

-   docker run mjw1200/lotto
-   docker run mjw1200/lotto:2.0

Even if you've deleted the Docker image locally. So that's nice.
