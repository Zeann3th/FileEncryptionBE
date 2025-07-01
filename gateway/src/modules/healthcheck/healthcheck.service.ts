import { Injectable } from '@nestjs/common';

@Injectable()
export class HealthcheckService {
  constructor() { }

  healthCheck() {
    return {
      message: "Server is running"
    }
  }
}
