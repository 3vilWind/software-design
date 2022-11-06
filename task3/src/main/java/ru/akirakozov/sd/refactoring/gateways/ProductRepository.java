package ru.akirakozov.sd.refactoring.gateways;

import ru.akirakozov.sd.refactoring.model.Product;
import ru.akirakozov.sd.refactoring.model.ProductIn;

import java.lang.reflect.Array;
import java.sql.*;
import java.util.ArrayList;

public class ProductRepository {
    protected static final String INSERT_SQL = "INSERT INTO PRODUCT (NAME, PRICE) VALUES (?, ?)";
    protected static final String SELECT_SQL = "SELECT * FROM PRODUCT";

    protected String databaseUrl;

    public ProductRepository(String databaseUrl) {
        this.databaseUrl = databaseUrl;
    }

    public void addProduct(ProductIn product) {
        try (Connection c = this.getConnection();
             PreparedStatement stmt = c.prepareStatement(INSERT_SQL)) {
            stmt.setString(1, product.name);
            stmt.setLong(2, product.price);
            stmt.executeUpdate();
        } catch (SQLException e) {
            throw new RuntimeException(e);
        }
    }

    public Product[] getAllProducts() {
        ArrayList<Product> result = new ArrayList<>();
        try (Connection c = this.getConnection();
             Statement stmt = c.createStatement();
             ResultSet rs = stmt.executeQuery(SELECT_SQL)) {

            while (rs.next()) {
                int id = rs.getInt("id");
                String name = rs.getString("name");
                int price = rs.getInt("price");
                result.add(new Product(id, name, price));
            }
        } catch (SQLException e) {
            throw new RuntimeException(e);
        }
        Product[] resultArray = (Product[]) Array.newInstance(Product.class, result.size());
        return result.toArray(resultArray);
    }

    protected Connection getConnection() throws SQLException {
        return DriverManager.getConnection(this.databaseUrl);
    }
}
