import { Controller, Get, HttpCode } from '@nestjs/common';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';
import { HealthcheckService } from './healthcheck.service';

@ApiTags("Healthcheck")
@Controller('healthz')
export class HealthcheckController {

  constructor(private readonly healthcheckService: HealthcheckService) { }

  @ApiOperation({ summary: "Health Check" })
  @ApiResponse({ status: 200, description: "Server is running" })
  @Get()
  healthCheck() {
    return this.healthcheckService.healthCheck();
  }
}
