# Open AI API KEy

This policy add the apikey to the request from a bucket of keys. This allows the user to provide several apikeys to avoid hitting any rate limit as the keys will be rotated and managed in a centrilized place

## Configuration
| Name    | Type  | Description                                                                                  | Default Value |
|---------|-------|----------------------------------------------------------------------------------------------|---------------|
| api_key | array | The API Key from open ai. It will use one from the list so that it avoids hitting rate limit |               |

## Example

No need for the `"Authorization: Bearer $OPENAI_API_KEY"`

```
>  curl http://localhost:8081/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [
      {
        "role": "system",
        "content": "You are ."
      },
      {
        "role": "user",
        "content": "Drivers license test"
      }
    ]
  }'
  
```

