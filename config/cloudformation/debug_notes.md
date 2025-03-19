# Editing Notes

- create [amazon erc](https://us-east-1.console.aws.amazon.com/ecr/private-registry/   repositories?region=us-east-1)
  -update `ImageArn` in complete
- HostedZone
- CertificateArn
- ExecutionRoleArn
- VPC

- taken out for now since we dont host the domain yet
```json
  "Parameters": {
    "HostedZone": {
      "NoEcho": false,
      "Description": "Hosted zone for DNS",
      "Type": "AWS::Route53::HostedZone::Id",
      "Default": "Z07055423Q2H8MJRS1SJ2"
    },


"DNSRecord": {
      "Type": "AWS::Route53::RecordSet",
      "Properties": {
        "HostedZoneId": {
          "Ref": "HostedZone"
        },
        "Name": {
          "Fn::Sub": "${Name}.slygames.org"
        },
        "Type": "A",
        "AliasTarget": {
          "HostedZoneId": {
            "Fn::GetAtt": ["LoadBalancer", "CanonicalHostedZoneID"]
          },
          "DNSName": {
            "Fn::GetAtt": ["LoadBalancer", "DNSName"]
          }
        }
      }
    }
```

<!-- # Enable Access Log for ELBv2

There is already an S3 bucket created for this purpose. The bucket name is `miviva-access-log`. You can use this bucket or create your own.
It was setup using instructions from [enable-access-logging](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/enable-access-logging.html)

You need to add the following LoadBalancerAttributes to the LoadBalancer resource to use it:

```json
{
  "LoadBalancer": {
    "Type": "AWS::ElasticLoadBalancingV2::LoadBalancer",
    "Properties": {
      "LoadBalancerAttributes": [
        {
          "Key": "access_logs.s3.enabled",
          "Value": "true"
        },
        {
          "Key": "access_logs.s3.bucket",
          "Value": "miviva-access-log"
        },
        {
          "Key": "access_logs.s3.prefix",
          "Value": "elb"
        }
      ]
    }
  }
}
```

Or set these manually

For how to ready the logs see these:

- [access-log-format](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-access-logs.html#access-log-entry-format)
- [502 errors](https://repost.aws/knowledge-center/elb-alb-troubleshoot-502-errors)
- [Load Balancer Troubleshooting](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-troubleshooting.html) for more infomation on how -->
