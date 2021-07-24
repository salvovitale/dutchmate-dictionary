### Run with docker

build image:
```bash
$ docker build -f Dockerfile -t "docker-repo/image-name:tag" .
```

run image:
```bash
$ docker run -p 8000:8000 -it --rm docker-repo/image-name:tag
```

push image:
```bash
$ docker push docker-repo/image-name:tag
```