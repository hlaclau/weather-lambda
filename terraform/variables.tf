variable "aws_region" {
  type    = string
  default = "eu-west-3"
}

variable "function_name" {
  type    = string
  default = "weather-lambda"
}

variable "discord_webhook_url" {
  type      = string
  sensitive = true
}

variable "schedule_expression" {
  type    = string
  default = "cron(0 7 * * ? *)"
}

variable "schedule_timezone" {
  type    = string
  default = "Europe/Paris"
}

variable "lambda_timeout" {
  type    = number
  default = 30
}

variable "lambda_memory_size" {
  type    = number
  default = 128
}

variable "log_retention_days" {
  type    = number
  default = 14
}
