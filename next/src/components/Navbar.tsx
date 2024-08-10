/**
 * v0 by Vercel.
 * @see https://v0.dev/t/RMdyrKkVVAE
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */
import Link from "next/link";
import { Button } from "@/components/ui/button";

export default function Navbar() {
  return (
    <div>
      <header className="flex h-20 w-full shrink-0 items-center px-4 md:px-6">
        <Link href="#" className="mr-6 hidden lg:flex" prefetch={false}>
          <MountainIcon className="h-6 w-6" />
          <span className="sr-only">Acme Inc</span>
        </Link>
        <div className="ml-auto flex gap-2">
          <Button variant="outline">Sign in</Button>
          <Button>Sign Up</Button>
        </div>
      </header>
      <section className="grid grid-cols-2 gap-4 p-4 md:grid-cols-4 md:p-6">
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 1"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 1</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 2"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 2</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 3"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 3</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
        <div className="relative overflow-hidden rounded-lg group">
          <Link href="#" className="absolute inset-0 z-10" prefetch={false}>
            <span className="sr-only">View</span>
          </Link>
          <img
            src="/placeholder.svg"
            alt="Product 4"
            width={400}
            height={300}
            className="object-cover w-full h-60"
            style={{ aspectRatio: "400/300", objectFit: "cover" }}
          />
          <div className="p-4 bg-background">
            <h3 className="text-lg font-semibold md:text-xl">Product 4</h3>
            <p className="text-sm text-muted-foreground">Description</p>
          </div>
        </div>
      </section>
    </div>
  );
}

function MountainIcon(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="m8 3 4 8 5-5 5 15H2L8 3z" />
    </svg>
  );
}
