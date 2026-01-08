export function LoadingSpinner() {
  return (
    <div className="flex h-full w-full items-center justify-center p-12">
      <div className="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent" />
    </div>
  )
}
