fn main() {
    // Configuração para o Windows usar a libmpv-2.lib
    #[cfg(windows)]
    {
        // Indica ao Cargo para procurar por libmpv-2.lib (ou .dll.a renomeada) na raiz do projeto
        println!("cargo:rustc-link-search=native=.");
        println!("cargo:rustc-link-lib=libmpv-2");
    }
}
