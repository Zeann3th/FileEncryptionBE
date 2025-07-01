import { Body, Controller, Get, HttpCode, Param, Patch, Post, Query, Req, Res, UseGuards } from '@nestjs/common';
import { AuthService } from './auth.service';
import { LoginUserDto, RegisterUserDto, UpdateUserDto } from './dto/auth.dto';
import { ApiBearerAuth, ApiBody, ApiCookieAuth, ApiOperation, ApiParam, ApiQuery, ApiTags } from '@nestjs/swagger';
import { Request, Response } from 'express';
import env from 'src/common';
import { JwtAuthGuard } from 'src/guards/jwt.guard';
import { RolesGuard } from 'src/guards/role.guard';
import { Roles } from 'src/decorators/role.decorator';
import { User } from 'src/decorators/user.decorator';
import { UserInterface } from 'src/common/types';

@ApiTags("Authentication")
@Controller('auth')
export class AuthController {

  constructor(private readonly authService: AuthService) { }

  @ApiOperation({ summary: "Register a new user" })
  @ApiBody({
    schema: {
      type: "object",
      properties: {
        username: { type: "string", example: "username" },
        name: { type: "string", example: "name" },
        password: { type: "string", example: "password" },
        email: { type: "string", example: "email" },
      },
      required: ["username", "password", "email"]
    }
  })
  @Post('sign-up')
  async register(@Body() body: RegisterUserDto) {
    return await this.authService.register(body);
  }

  @ApiOperation({ summary: "Login a user" })
  @ApiBody({
    schema: {
      type: "object",
      properties: {
        username: { type: "string", example: "username" },
        password: { type: "string", example: "password" }
      },
      required: ["password"]
    }
  })
  @Post('sign-in')
  @HttpCode(200)
  async login(@Body() body: LoginUserDto, @Res() response: Response) {
    const { accessToken, refreshToken } = await this.authService.login(body);
    response.cookie("refresh_token", refreshToken, {
      httpOnly: true,
      secure: env.NODE_ENV === "production",
      maxAge: 1000 * 60 * 60 * 24 * 7,
      path: "/",
      sameSite: env.NODE_ENV === "production" ? "none" : "lax",
      partitioned: env.NODE_ENV === "production",
    });
    return response.send({ access_token: accessToken });
  }

  @ApiOperation({ summary: "Refresh access token" })
  @ApiCookieAuth("refresh_token")
  @Get('refresh')
  async refresh(@Req() request: Request) {
    const refreshToken = request.cookies["refresh_token"];
    return await this.authService.refresh(refreshToken);
  }

  @ApiOperation({ summary: "Logout a user" })
  @ApiCookieAuth("refresh_token")
  @Get('logout')
  @HttpCode(204)
  async logout(@Req() request: Request, @Res() response: Response) {
    const refreshToken = request.cookies["refresh_token"];
    await this.authService.logout(refreshToken);
    response.clearCookie("refresh_token", {
      httpOnly: true,
      secure: env.NODE_ENV === "production",
      maxAge: 1000 * 60 * 60 * 24 * 7,
      path: "/",
      sameSite: env.NODE_ENV === "production" ? "none" : "lax",
      partitioned: env.NODE_ENV === "production",
    });
    return response.status(204).send();
  }

  @ApiOperation({ summary: "Update user credentials and privileges" })
  @ApiParam({ name: "id", description: "User id" })
  @ApiBody({
    schema: {
      type: "object",
      properties: {
        name: { type: "string", example: "name" },
        email: { type: "string", example: "email" },
        role: { type: "string", example: "ADMIN" }
      },
      required: []
    }
  })
  @ApiBearerAuth()
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles("ADMIN")
  @Patch(":id")
  async update(@Param("id") id: string, @Body() body: UpdateUserDto) {
    return await this.authService.update(id, body);
  }

  @ApiOperation({ summary: "Search user by name, email" })
  @ApiBearerAuth()
  @ApiQuery({ name: "name", required: false, type: String })
  @ApiQuery({ name: "email", required: false, type: String })
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles("ADMIN", "SECURITY")
  @Get("search")
  async search(@Query("name") name: string, @Query("email") email: string) {
    return await this.authService.search(name, email);
  }

  @ApiOperation({ summary: "Get user by id" })
  @ApiBearerAuth()
  @ApiParam({ name: "id", description: "User id" })
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles("ADMIN", "SECURITY", "USER")
  @Get(":id")
  async getById(@User() user: UserInterface, @Param("id") id: string) {
    return await this.authService.getById(user, id);
  }
}
