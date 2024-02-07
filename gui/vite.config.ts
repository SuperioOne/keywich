import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vitest/config';

export function swap_package(from: string, target: string) {
  const regex = new RegExp(`import.{0,500} from ["'](${from}).{0,500}["']`);
  return {
    name: 'swap_package',
    transform(src: string, id: string) {
      const match = regex.exec(src);
      if (match !== null && match[1] !== undefined) {
        const [allLine, part] = match;
        const newImport = allLine.replace(`/${from}/`, `/${target}/`);
        
        return {
          code: src.replace(allLine, newImport),
          map: null,
        }
      }
    },
  }
}

// swap_package("@keywich/memory_api", "@keywich/memory_api")
export default defineConfig({
  plugins: [sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
  }
});
