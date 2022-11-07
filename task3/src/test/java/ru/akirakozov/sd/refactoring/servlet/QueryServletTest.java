package ru.akirakozov.sd.refactoring.servlet;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;
import ru.akirakozov.sd.refactoring.Utils;
import ru.akirakozov.sd.refactoring.gateways.HTMLRenderer;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.sql.Connection;
import java.sql.SQLException;
import java.sql.Statement;

import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

public class QueryServletTest {
    @Mock
    private HttpServletRequest request;
    @Mock
    private HttpServletResponse response;

    private QueryServlet servlet;
    private AutoCloseable closeable;

    @BeforeEach
    void setUp() throws SQLException {
        closeable = MockitoAnnotations.openMocks(this);
        servlet = new QueryServlet(Utils.getProductServiceForTestDatabase(), new HTMLRenderer());
        Utils.cleanTestDatabase();

        try (Connection c = Utils.getTestDatabase();
             Statement stmt = c.createStatement()) {
            stmt.executeUpdate("INSERT INTO PRODUCT VALUES " +
                    "(1, 'cookies', 100)," +
                    "(2, 'memes', 300)," +
                    "(3, 'tea', 150)");
        }
    }

    @AfterEach
    void tearDown() throws Exception {
        closeable.close();
    }

    @Test
    void testMaxQuery() throws IOException {
        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));
        when(request.getParameter("command")).thenReturn("max");

        servlet.doGet(request, response);

        String[] resp = {"<h1>Product with max price: </h1>", "memes\t300</br>"};
        Assertions.assertArrayEquals(Utils.getCommonHtmlStyling(resp), stringWriter.toString().split("\r\n|\n"));
        verify(response).setStatus(HttpServletResponse.SC_OK);
    }

    @Test
    void testMinQuery() throws IOException {
        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));
        when(request.getParameter("command")).thenReturn("min");

        servlet.doGet(request, response);

        String[] resp = {"<h1>Product with min price: </h1>", "cookies\t100</br>"};
        Assertions.assertArrayEquals(Utils.getCommonHtmlStyling(resp), stringWriter.toString().split("\r\n|\n"));
        verify(response).setStatus(HttpServletResponse.SC_OK);
    }

    @Test
    void testSumQuery() throws IOException {
        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));
        when(request.getParameter("command")).thenReturn("sum");

        servlet.doGet(request, response);

        String[] resp = {"Summary price: ", "550"};
        Assertions.assertArrayEquals(Utils.getCommonHtmlStyling(resp), stringWriter.toString().split("\r\n|\n"));
        verify(response).setStatus(HttpServletResponse.SC_OK);
    }

    @Test
    void testCountQuery() throws IOException {
        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));
        when(request.getParameter("command")).thenReturn("count");

        servlet.doGet(request, response);

        String[] resp = {"Number of products: ", "3"};
        Assertions.assertArrayEquals(Utils.getCommonHtmlStyling(resp), stringWriter.toString().split("\r\n|\n"));
        verify(response).setStatus(HttpServletResponse.SC_OK);
    }

    @Test
    void testUnknownQuery() throws IOException {
        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));
        when(request.getParameter("command")).thenReturn("lolkek");

        servlet.doGet(request, response);

        Assertions.assertEquals(stringWriter.toString().trim(), "Unknown command: lolkek");
        verify(response).setStatus(HttpServletResponse.SC_OK);
    }
}
