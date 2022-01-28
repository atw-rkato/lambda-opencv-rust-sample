# lambda-opencv-rust-sample

## コンテナイメージをECRにpushする

```shell 
aws ecr get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin 012345678901.dkr.ecr.ap-northeast-1.amazonaws.com
docker build -t lambda-opencv-rust-sample . --build-arg ACCOUNT_ID=012345678901 --progress plain
docker tag lambda-opencv-rust-sample:latest 012345678901.dkr.ecr.ap-northeast-1.amazonaws.com/lambda-opencv-rust-sample:latest
docker push 012345678901.dkr.ecr.ap-northeast-1.amazonaws.com/lambda-opencv-rust-sample:latest
```
