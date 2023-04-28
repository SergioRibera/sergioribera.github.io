import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';

export class CdkStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);
        // Lambda Backend <-> DynamoDB
        // Lambda Backend <-> s3
        //
        // Lambda Web <- Lambda Backend Service
        // Lambda Web Admin <-> Lambda Backend Service
    }
}
