#!/bin/bash

echo "Simple test:"
echo "-----------"
echo "test 1: Should succeed:"
curl -X POST -d 'token=token&team_id=team_id&team_domain=team_domain&enterprise_id=enterprise_id&enterprise_name=enterprise_name&channel_id=channel_id&channel_name=channel_name&user_id=user_id&user_name=user_name&command=command&text=add+vacation+today&response_url=response_url&trigger_id=trigger_id&api_app_id=api_app_id' localhost:8000
echo
echo "---"
echo "test 2: Should fail:"
curl -X POST -d 'token=token&team_id=team_id&team_domain=team_domain&enterprise_id=enterprise_id&enterprise_name=enterprise_name&channel_id=channel_id&channel_name=channel_name&user_id=user_id&user_name=user_name&command=command&text=add+vacati+today&response_url=response_url&trigger_id=trigger_id&api_app_id=api_app_id' localhost:8000
echo