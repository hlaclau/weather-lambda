output "lambda_function_arn" {
  value = aws_lambda_function.weather.arn
}

output "lambda_function_name" {
  value = aws_lambda_function.weather.function_name
}

output "scheduler_arn" {
  value = aws_scheduler_schedule.daily_weather.arn
}

output "log_group_name" {
  value = aws_cloudwatch_log_group.lambda.name
}
