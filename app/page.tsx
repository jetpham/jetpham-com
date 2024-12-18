import { BlogPosts } from "app/components/posts";

export default function Page() {
  return (
    <section>
      <h1 className="mb-8 text-2xl font-semibold tracking-tighter">
        My Website
      </h1>
      <p className="mb-4">
        {`I am Jet Pham, a second year computer science student at the University of San Francisco. I am interested in rust, competitive programming, robotics, nextjs, arts & crafts, and food.`}
      </p>
      <div className="my-8">
        <BlogPosts />
      </div>
    </section>
  );
}
