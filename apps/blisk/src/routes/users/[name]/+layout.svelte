<script lang="ts">
  import { base } from "$app/paths";
  import { page } from "$app/stores";
  import Link from "./Link.svelte";

  const { data, children } = $props();

  const days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

  const basePath = $derived<`/${string}`>(`${base}/users/${$page.params.name}`);
</script>

<div class="mx-auto flex w-full flex-col gap-8 p-4 md:p-10">
  <div>
    <img
      src="/AGI2.webp"
      alt=""
      height={384}
      class="h-48 w-full select-none rounded-3xl object-cover shadow-lg transition-all duration-150 md:h-60 lg:h-96"
    />
    <div class="flex flex-row items-center gap-8 px-8">
      <img
        src="/no-avatar.webp"
        class="border-border-light dark:border-border-dark -mt-8 h-auto w-[150px] select-none rounded-full border shadow-lg"
        alt=""
      />
      <h1>
        <span class="sr-only">The home of</span>
        {data.data.name}
      </h1>
    </div>
  </div>
  <div class="flex w-full flex-col gap-8 md:gap-14 lg:flex-row-reverse">
    <div class="flex h-fit flex-[2_2_0] flex-col gap-4 lg:sticky lg:top-14">
      <h2>About {data.data.name}</h2>
      <div class="box md">here comes a quote</div>
      <div class="box md">
        69 books read this year
        <div class="w-full overflow-x-auto">
          <table class="w-max table-auto border-separate border-spacing-[4px] text-sm">
            <thead>
              <tr>
                <td class="w-7"><span class="sr-only">Day of week</span></td>
                <td colspan="5">Jan</td>
                <td colspan="4">Feb</td>
                <td colspan="5">Mar</td>
                <td colspan="5">Apr</td>
                <td colspan="5">May</td>
                <td colspan="5">Jun</td>
                <td colspan="5">Jul</td>
                <td colspan="5">Aug</td>
                <td colspan="5">Sep</td>
                <td colspan="5">Oct</td>
                <td colspan="5">Nov</td>
                <td colspan="5">Dec</td>
              </tr>
            </thead>
            <tbody>
              {#each Array.from({ length: 7 }).map((_, idx) => idx) as day}
                <tr class="h-10 md:h-[15px]">
                  <th class="relative" scope="row"><span class="absolute bottom-[-4px] left-0 font-normal">{days[day]}</span></th>
                  {#each Array.from({ length: 59 }) as _}
                    <td class="w-10 rounded-md bg-green-400 md:w-[15px] md:rounded-sm"></td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>
    <article class="flex flex-[3_3_0] flex-col gap-4">
      <h2>Recent activities</h2>
      <div class="flex h-max w-full flex-row flex-wrap justify-stretch gap-2">
        <Link href={basePath}>Reviews</Link>
        <Link href={`${basePath}/comments`}>Comments</Link>
        <Link href={`${basePath}/likes`}>Likes</Link>
        <Link href={`${basePath}/dislikes`}>Dislikes</Link>
      </div>
      <div>
        {@render children()}
      </div>
    </article>
  </div>
</div>
