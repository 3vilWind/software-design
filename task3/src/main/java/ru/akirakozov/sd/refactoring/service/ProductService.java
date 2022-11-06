package ru.akirakozov.sd.refactoring.service;

import ru.akirakozov.sd.refactoring.gateways.ProductRepository;
import ru.akirakozov.sd.refactoring.model.ProductIn;

public class ProductService {
    protected ProductRepository productRepository;

    public ProductService(ProductRepository productRepository) {
        this.productRepository = productRepository;
    }

    public void addProduct(ProductIn data) {
        this.productRepository.addProduct(data);
    }
//
//    public Product[] getAllProducts() {}
//
//    public Product getCheapestProduct() {}
//
//    public Product getMostExpensiveProduct() {}
//
//    public long getPriceOfAllProducts() {}
//
//    public int getProductsCount() {}
}
