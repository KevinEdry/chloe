export function HeroTiledBackground() {
    return       <div
        className="absolute inset-0 z-0"
        style={{
            backgroundImage: `url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='70' height='70'%3E%3Cpath d='M70 0 V70 H0' fill='none' stroke='%23a855f7' stroke-opacity='0.3'/%3E%3C/svg%3E")`,
            backgroundSize: '70px 70px',
            backgroundPosition: 'center bottom',
            backgroundRepeat: 'repeat',
        }}
    />

}