package ru.akirakozov.sd.refactoring;

import org.eclipse.jetty.server.Server;
import org.eclipse.jetty.servlet.ServletContextHandler;
import org.eclipse.jetty.servlet.ServletHolder;
import ru.akirakozov.sd.refactoring.gateways.HTMLRenderer;
import ru.akirakozov.sd.refactoring.gateways.ProductRepository;
import ru.akirakozov.sd.refactoring.gateways.ProductSQLRepository;
import ru.akirakozov.sd.refactoring.gateways.Renderer;
import ru.akirakozov.sd.refactoring.service.ProductService;
import ru.akirakozov.sd.refactoring.servlet.AddProductServlet;
import ru.akirakozov.sd.refactoring.servlet.GetProductsServlet;
import ru.akirakozov.sd.refactoring.servlet.QueryServlet;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.Statement;

/**
 * @author akirakozov
 */
public class Main {
    static final String DATABASE_URL = "jdbc:sqlite:test.db";

    public static void main(String[] args) throws Exception {
        ProductRepository productRepository = new ProductSQLRepository("jdbc:sqlite:test.db");
        ProductService productService = new ProductService(productRepository);
        Renderer renderer = new HTMLRenderer();

        try (Connection c = DriverManager.getConnection(DATABASE_URL)) {
            String sql = "CREATE TABLE IF NOT EXISTS PRODUCT" +
                    "(ID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL," +
                    " NAME           TEXT    NOT NULL, " +
                    " PRICE          INT     NOT NULL)";
            Statement stmt = c.createStatement();

            stmt.executeUpdate(sql);
            stmt.close();
        }

        Server server = new Server(8081);

        ServletContextHandler context = new ServletContextHandler(ServletContextHandler.SESSIONS);
        context.setContextPath("/");
        server.setHandler(context);

        context.addServlet(new ServletHolder(new AddProductServlet(productService, renderer)), "/add-product");
        context.addServlet(new ServletHolder(new GetProductsServlet(productService, renderer)),"/get-products");
        context.addServlet(new ServletHolder(new QueryServlet(productService, renderer)),"/query");

        server.start();
        server.join();
    }
}
