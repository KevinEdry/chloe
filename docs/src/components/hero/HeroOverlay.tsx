export function HeroOverlay(){
    return <div
        className="absolute inset-0 z-[1]"
        style={{
            backgroundImage:
                'linear-gradient(180deg, rgba(10, 10, 15, 1) 0%, rgba(10, 10, 15, 0) 100%)',
        }}
    />
}