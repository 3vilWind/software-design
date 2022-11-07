package ru.akirakozov.sd.refactoring.service;

import ru.akirakozov.sd.refactoring.gateways.ProductRepository;
import ru.akirakozov.sd.refactoring.gateways.ProductSQLRepository;
import ru.akirakozov.sd.refactoring.model.Product;
import ru.akirakozov.sd.refactoring.model.ProductIn;
import ru.akirakozov.sd.refactoring.model.query.*;

public class ProductService {
    public enum QueryType {
        CHEAPEST_PRODUCT,
        MOST_EXPENSIVE_PRODUCT,
        PRICE_OF_ALL_PRODUCTS,
        PRODUCTS_COUNT
    }

    protected ProductRepository productRepository;

    public ProductService(ProductRepository productRepository) {
        this.productRepository = productRepository;
    }

    public void addProduct(ProductIn data) {
        this.productRepository.addProduct(data);
    }

    public Product[] getAllProducts() {
        return this.productRepository.getAllProducts();
    }

    public QueryResult query(QueryType queryType) {
        return switch (queryType) {
            case CHEAPEST_PRODUCT -> new MinPriceProductResult(this.productRepository.getCheapestProduct());
            case MOST_EXPENSIVE_PRODUCT -> new MaxPriceProductResult(this.productRepository.getMostExpensiveProduct());
            case PRICE_OF_ALL_PRODUCTS -> new SumPriceResult(this.productRepository.getPriceOfAllProducts());
            case PRODUCTS_COUNT -> new ProductsCountResult(this.productRepository.getProductsCount());
        };
    }
}
