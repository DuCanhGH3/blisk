import type { Handle } from "@sveltejs/kit";
import { PrismaClient } from "@prisma/client";
import { PrismaD1 } from "@prisma/adapter-d1";
import jwt from "jsonwebtoken";
import { JWT_SECRET } from "$env/static/private";
import { A_MONTH_IN_SECONDS } from "$lib/constants";

let prisma: PrismaClient | null = null;

export const handle: Handle = async ({ event, resolve }) => {
  if (prisma === null) {
    if (!event.platform?.env?.DATABASE) {
      throw new Error("platform.env.DATABASE is not defined.");
    }
    const adapter = new PrismaD1(event.platform.env.DATABASE);
    prisma = new PrismaClient({ adapter });
  }
  event.locals.prisma = prisma;
  const token = event.cookies.get("token");
  if (token) {
    try {
      const decodedToken = jwt.verify(token, JWT_SECRET, { maxAge: A_MONTH_IN_SECONDS });
      if (typeof decodedToken === "object" && "id" in decodedToken) {
        const user = await prisma.user.findFirst({
          where: {
            id: decodedToken.id,
          },
          select: {
            email: true,
            name: true,
          },
        });
        event.locals.user = user;
      }
    } catch (err) {
      console.log("An error occurred while fetching user", err);
    }
  }
  return resolve(event);
};
