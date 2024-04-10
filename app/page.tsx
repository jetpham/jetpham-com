import { BlogPosts } from "app/components/posts";

export default function Page() {
  return (
    <section>
      <h1 className="mb-8 text-2xl font-semibold tracking-tighter">
        My Portfolio
      </h1>
      <p className="mb-4">
        {`I am Jet Pham, a first year computer science student at the University of San Francisco. I am interested rust, linux, robotics, arts & crafts, and food.`}
      </p>
      <div className="my-8">
        <BlogPosts />
      </div>
    </section>
  );
}
