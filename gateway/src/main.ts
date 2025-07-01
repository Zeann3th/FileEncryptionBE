import env from './common';
import { NestFactory } from '@nestjs/core';
import { AppModule } from './modules/app.module';
import { Logger, ValidationPipe } from '@nestjs/common';
import { NestExpressApplication } from '@nestjs/platform-express';
import helmet from 'helmet';
import * as morgan from 'morgan';
import * as compression from 'compression';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';
import * as cookieParser from 'cookie-parser';

const setMiddleware = (app: NestExpressApplication) => {
  app.set("trust proxy", 1);

  app.use(compression());

  app.use(helmet());

  app.enableCors({
    origin: "*",
    credentials: true,
    allowedHeaders: ["Content-Type", "Authorization", "Cache-Control"],
    methods: ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"],
  });

  app.use(morgan("combined"));

  app.use(cookieParser());

  app.useGlobalPipes(new ValidationPipe({
    transform: true,
    whitelist: true,
  }));

};

async function bootstrap() {
  const app = await NestFactory.create<NestExpressApplication>(AppModule, {
    logger: new Logger("[]"),
  });

  const logger = new Logger("APP");
  app.useLogger(logger);

  app.setGlobalPrefix("v1");

  setMiddleware(app);

  if (env.NODE_ENV !== "production") {
    const swaggerConfig = new DocumentBuilder()
      .setTitle("Parking Lot")
      .setDescription('API documentation for Parking Lot Backend')
      .setVersion('alpha')
      .addBearerAuth()
      .build();

    const swaggerDocument = SwaggerModule.createDocument(app, swaggerConfig);
    SwaggerModule.setup('swagger', app, swaggerDocument, {
      jsonDocumentUrl: 'swagger/json',
    });
  }


  await app.listen(env.GATEWAY_PORT, () => {
    logger.log(`🚀 File Encyption Gateway 0.1.0 on http://localhost:${env.GATEWAY_PORT}`);
  });
}

bootstrap();
