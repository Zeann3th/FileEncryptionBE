import { Injectable, CanActivate, ExecutionContext, ForbiddenException } from '@nestjs/common';
import { Reflector } from '@nestjs/core';
import { UserInterface } from 'src/common/types';

@Injectable()
export class RolesGuard implements CanActivate {
  constructor(
    private reflector: Reflector,
  ) { }

  async canActivate(context: ExecutionContext): Promise<boolean> {
    const requiredRoles = this.reflector.get<string[]>(
      'roles',
      context.getHandler(),
    );
    if (!requiredRoles) return true;

    const request = context.switchToHttp().getRequest();
    const user: UserInterface = request.user;

    if (!user || !requiredRoles.includes(user.role)) {
      throw new ForbiddenException({
        message: 'You do not have permission to access this resource',
      });
    }

    request.user = user;

    return true;
  }
}
