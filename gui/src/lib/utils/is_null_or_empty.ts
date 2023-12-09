export function is_null_or_empty(value: string | any[] | null | undefined): value is undefined | null {
  switch (typeof value) {
    case "undefined":
      return true;
    case "object":
      return value === null || value?.length < 1;
    case "string":
      return value.trim().length < 1;
  }
}