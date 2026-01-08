export function AppLoading() {
    return (
        <div className="flex h-screen w-screen items-center justify-center bg-background">
            <div className="flex flex-col items-center gap-4">
                <div className="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent" />
                <div className="text-sm text-muted-foreground">
                    Loading Dashboard...
                </div>
            </div>
        </div>
    );
}
