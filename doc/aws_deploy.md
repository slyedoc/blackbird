# AWS Deployment

## AWS Cloud formation

See [What is AWS Cloudformation](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/Welcome.html) for more information.

### Create Stack

To create a stack, run the following command:

```bash
aws cloudformation create-stack --stack-name blackbird-{name} --template-body file://./config/cloudformation/template.json --capabilities CAPABILITY_NAMED_IAM --parameters ParameterKey=DBPassword,ParameterValue={dbPassword} ParameterKey=Name,ParameterValue={name}
```

```bash
aws cloudformation create-stack --stack-name blackbird-prod --template-body file://./config/cloudformation/template.json --capabilities CAPABILITY_NAMED_IAM --parameters ParameterKey=DBPassword,ParameterValue=nkunku12 ParameterKey=Name,ParameterValue=prod
```

> Replace {name}, {dbPassword}
> Version is based on github tag, for example: v4.1.3

### Update Stack

To update a stack, run the following command:

```bash
aws cloudformation update-stack --stack-name miviva-{name} --template-body file://./config/cloudformation/template.json --capabilities CAPABILITY_NAMED_IAM --parameters ParameterKey=DBPassword,UsePreviousValue=true ParameterKey=Name,UsePreviousValue=true ParameterKey=AdminPassword,UsePreviousValue=true ParameterKey=ImageArn,ParameterValue=440026511511.dkr.ecr.us-east-1.amazonaws.com/miviva-ecr:{version}
```

> Replace {name} and {version} with the correct values

After the stack is updated, you will need to force a new deployment of the ECS service.

```bash
aws ecs update-service --cluster miviva-{name}-cluster --service miviva-{name}-service --force-new-deployment
```

> Replace {name} with the correct values

### Delete Stack

<strong>Be sure you have a backup of files and db first!!!!!</strong>

To delete a stack, run the following command:

```bash
aws cloudformation delete-stack --stack-name miviva-{name}
```

## Notes on Testing Locally

Build Site

```bash
pnpm nuxt build
```

Run Locally, requires env file for settings

```bash
dotenv -e .env.aws.local node .output/index.js
```

## Build Image

Build Docker Image

```bash
docker build --tag miviva_webserver --no-cache --file ./config/webserver/Dockerfile .  --progress=plain
```

Start Docker Image Locally

```bash
docker run --env-file ./.env.aws.prod -it --publish 80:80  miviva_webserver sh
```

> Note: This assumes port 80 is available on the host machine.
