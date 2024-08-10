/**
 * v0 by Vercel.
 * @see https://v0.dev/t/DHoPpuIM9Li
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */
import Link from "next/link";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";

export default function Component() {
  return (
    <div className="bg-muted py-12 md:py-24">
      <div className="container grid gap-6 px-4 md:gap-8 md:px-6">
        <div className="flex flex-col items-center gap-4 md:flex-row md:justify-between">
          <div className="grid gap-1">
            <h2 className="text-2xl font-bold tracking-tight">Pricing</h2>
            <p className="text-muted-foreground">
              Choose the plan that's right for you.
            </p>
          </div>
          <Link
            href="#"
            className="inline-flex h-10 items-center justify-center rounded-md bg-primary px-6 text-sm font-medium text-primary-foreground shadow transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
            prefetch={false}
          >
            Shop Now
          </Link>
        </div>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-5">
          <Card className="bg-[#e5e7eb]">
            <CardHeader className="bg-[#e5e7eb]">
              <CardTitle>Platinum</CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-2">
                <h3 className="text-4xl font-bold">$99</h3>
                <p className="text-muted-foreground">per month</p>
              </div>
            </CardContent>
            <CardFooter>
              <Button>Select</Button>
            </CardFooter>
          </Card>
          <Card className="bg-[#fcd34d]">
            <CardHeader className="bg-[#fcd34d]">
              <CardTitle>Gold</CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-2">
                <h3 className="text-4xl font-bold">$49</h3>
                <p className="text-muted-foreground">per month</p>
              </div>
            </CardContent>
            <CardFooter>
              <Button>Select</Button>
            </CardFooter>
          </Card>
          <Card className="bg-[#a3a3a3]">
            <CardHeader className="bg-[#a3a3a3]">
              <CardTitle>Silver</CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-2">
                <h3 className="text-4xl font-bold">$29</h3>
                <p className="text-muted-foreground">per month</p>
              </div>
            </CardContent>
            <CardFooter>
              <Button>Select</Button>
            </CardFooter>
          </Card>
          <Card className="bg-[#cd7f32]">
            <CardHeader className="bg-[#cd7f32]">
              <CardTitle>Bronze</CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-2">
                <h3 className="text-4xl font-bold">$19</h3>
                <p className="text-muted-foreground">per month</p>
              </div>
            </CardContent>
            <CardFooter>
              <Button>Select</Button>
            </CardFooter>
          </Card>
          <Card className="bg-[#8b4513]">
            <CardHeader className="bg-[#8b4513]">
              <CardTitle>Wood</CardTitle>
            </CardHeader>
            <CardContent className="p-6">
              <div className="space-y-2">
                <h3 className="text-4xl font-bold">$9</h3>
                <p className="text-muted-foreground">per month</p>
              </div>
            </CardContent>
            <CardFooter>
              <Button>Select</Button>
            </CardFooter>
          </Card>
        </div>
      </div>
    </div>
  );
}
