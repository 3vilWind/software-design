package ru.akirakozov.sd.refactoring.gateways;

import ru.akirakozov.sd.refactoring.model.Product;
import ru.akirakozov.sd.refactoring.model.ProductIn;

import java.lang.reflect.Array;
import java.sql.*;
import java.util.ArrayList;

public class ProductSQLRepository implements ProductRepository {
    protected String databaseUrl;

    public ProductSQLRepository(String databaseUrl) {
        this.databaseUrl = databaseUrl;
    }

    public void addProduct(ProductIn product) {
        try (Connection c = this.getConnection();
             PreparedStatement stmt = c.prepareStatement("INSERT INTO PRODUCT (NAME, PRICE) VALUES (?, ?)")) {
            stmt.setString(1, product.name);
            stmt.setLong(2, product.price);
            stmt.executeUpdate();
        } catch (SQLException e) {
            throw new RuntimeException(e);
        }
    }

    public Product[] getAllProducts() {
        return queryMultipleProducts("SELECT * FROM PRODUCT");
    }

    public Product getCheapestProduct() {
        return queryFirstProduct("SELECT * FROM PRODUCT ORDER BY PRICE LIMIT 1");
    }

    public Product getMostExpensiveProduct() {
        return queryFirstProduct("SELECT * FROM PRODUCT ORDER BY PRICE DESC LIMIT 1");
    }

    public int getPriceOfAllProducts() {
        return queryFirstInt("SELECT SUM(price) FROM PRODUCT");
    }

    public int getProductsCount() {
        return queryFirstInt("SELECT COUNT(*) FROM PRODUCT");
    }

    protected Connection getConnection() throws SQLException {
        return DriverManager.getConnection(this.databaseUrl);
    }

    protected Product[] queryMultipleProducts(String sql) {
        ArrayList<Product> result = new ArrayList<>();
        try (Connection c = this.getConnection();
             Statement stmt = c.createStatement();
             ResultSet rs = stmt.executeQuery(sql)) {

            while (rs.next()) {
                result.add(deserializeProduct(rs));
            }
        } catch (SQLException e) {
            throw new RuntimeException(e);
        }
        Product[] resultArray = (Product[]) Array.newInstance(Product.class, result.size());
        return result.toArray(resultArray);
    }

    protected Product queryFirstProduct(String sql) {
        try (Connection c = this.getConnection();
             Statement stmt = c.createStatement();
             ResultSet rs = stmt.executeQuery(sql)) {

            if (rs.next()) {
                return deserializeProduct(rs);
            }
        } catch (SQLException e) {
            throw new RuntimeException(e);
        }
        throw new RuntimeException("There is no such element");
    }

    protected int queryFirstInt (String sql) {
        try (Connection c = this.getConnection();
             Statement stmt = c.createStatement();
             ResultSet rs = stmt.executeQuery(sql)) {

            if (rs.next()) {
                return rs.getInt(1);
            }
        } catch (SQLException e) {
            throw new RuntimeException(e);
        }
        throw new RuntimeException("There is no such element");
    }

    protected static Product deserializeProduct(ResultSet rs) throws SQLException {
        int id = rs.getInt("id");
        String name = rs.getString("name");
        int price = rs.getInt("price");
        return new Product(id, name, price);
    }
}
