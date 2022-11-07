package ru.akirakozov.sd.refactoring.gateways;

import ru.akirakozov.sd.refactoring.model.Product;
import ru.akirakozov.sd.refactoring.model.ProductIn;

import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.SQLException;

public interface ProductRepository {
    void addProduct(ProductIn product);

    Product[] getAllProducts();

    Product getCheapestProduct();

    Product getMostExpensiveProduct();

    int getPriceOfAllProducts();

    int getProductsCount();
}
